;; cljn.pedestal.chain — execução síncrona de cadeia de interceptors (ADR-0013).
;;
;; Módulo built-in de propriedade do compilador. Um interceptor é um mapa
;; {:name k :enter fn :leave fn :error fn}; cada estágio é opcional. Um contexto
;; (mapa com :request/:response) é encadeado pela fase :enter em ordem e pela fase
;; :leave em ordem inversa. Um :response presente após um :enter termina a fase de
;; entrada (ADR-0013 §3/§10). Uma exceção em :enter/:leave vira o erro do contexto,
;; e os handlers :error dos interceptors já entrados rodam em ordem inversa; um
;; handler pode recuperar produzindo um contexto (tipicamente com :response). Não
;; há preguiça, threads ou async neste subconjunto P1.
(ns cljn.pedestal.chain)

(defn- enter-all
  "Fase :enter: encadeia `ctx` pelos :enter de `interceptors` a partir do índice
  `i`, empilhando cada interceptor entrado em `stack` (ordem inversa). Para quando
  um :response está presente, quando um :enter lança (captura o erro no contexto) ou
  ao fim. Devolve {:ctx ... :stack ...}."
  [ctx interceptors i n stack]
  (if (or (>= i n) (get ctx :response))
    {:ctx ctx :stack stack}
    (let [ic (nth interceptors i)
          f (get ic :enter)]
      (if (nil? f)
        (recur ctx interceptors (inc i) n (cons ic stack))
        (let [r (try {:ctx (f ctx)} (catch E e {:err e}))]
          (if (get r :err)
            ;; O interceptor que lançou NÃO entra na pilha; seu próprio :error não
            ;; trata o próprio :enter (semântica do Pedestal).
            {:ctx (assoc ctx :cljn.chain/error (get r :err)) :stack stack}
            (recur (get r :ctx) interceptors (inc i) n (cons ic stack))))))))

(defn- leave-all
  "Fase :leave/:error: consome a `stack` (já em ordem inversa). Sem erro no
  contexto, roda o :leave de cada interceptor; com erro, roda o :error (que recebe
  [contexto erro] e pode recuperar devolvendo um contexto sem erro). Exceções em
  :leave/:error viram o erro do contexto."
  [ctx stack]
  (if (empty? stack)
    ctx
    (let [ic (first stack)
          err (get ctx :cljn.chain/error)]
      (if (nil? err)
        (let [h (get ic :leave)
              nctx (if (nil? h)
                     ctx
                     (try (h ctx) (catch E e (assoc ctx :cljn.chain/error e))))]
          (recur nctx (rest stack)))
        (let [g (get ic :error)]
          (if (nil? g)
            (recur ctx (rest stack))
            (recur (try (g (dissoc ctx :cljn.chain/error) err)
                        (catch E e (assoc ctx :cljn.chain/error e)))
                   (rest stack))))))))

(defn execute
  "Executa a `interceptors` (vetor) sobre o `context` (mapa). Devolve o contexto
  final. Se um erro permanecer não-recuperado ao fim, ele é relançado."
  [context interceptors]
  (let [n (count interceptors)
        entered (enter-all context interceptors 0 n (list))
        ctx (leave-all (get entered :ctx) (get entered :stack))]
    (if (get ctx :cljn.chain/error)
      (throw (get ctx :cljn.chain/error))
      ctx)))
