(ns cormen.structures.circular-queue)

(defn queue-workload [seed]
  (loop [step 0
         head 0
         tail 0
         size 0
         data [0 0 0 0 0 0 0 0]
         checksum 0]
    (if (>= step 48)
      (+ checksum size head tail)
      (if (or (< size 4) (= (mod (+ step seed) 3) 0))
        (let [value (+ seed (* step 7))]
          (recur (inc step)
                 head
                 (mod (inc tail) 8)
                 (inc size)
                 (assoc data tail value)
                 checksum))
        (recur (inc step)
               (mod (inc head) 8)
               tail
               (dec size)
               data
               (+ checksum (nth data head)))))))

(defn benchmark [rounds]
  (loop [n rounds checksum 0]
    (if (> n 0)
      (recur (dec n) (+ checksum (queue-workload (mod n 13))))
      checksum)))

(defn -main [] (println (benchmark 7000)))
(-main)
