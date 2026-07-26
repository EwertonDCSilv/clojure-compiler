(ns cormen.structures.binary-search-tree)

(defn insert-node [tree value]
  (if (nil? tree)
    [value nil nil]
    (let [key (nth tree 0)]
      (if (< value key)
        [key (insert-node (nth tree 1) value) (nth tree 2)]
        [key (nth tree 1) (insert-node (nth tree 2) value)]))))

(defn build-tree [values]
  (loop [i 0 tree nil]
    (if (< i (count values))
      (recur (inc i) (insert-node tree (nth values i)))
      tree)))

(defn tree-search [tree target]
  (cond
    (nil? tree) 0
    (= (nth tree 0) target) 1
    (< target (nth tree 0)) (recur (nth tree 1) target)
    :else (recur (nth tree 2) target)))

(defn depth-checksum [tree depth]
  (if (nil? tree)
    0
    (+ (* depth (nth tree 0))
       (depth-checksum (nth tree 1) (inc depth))
       (depth-checksum (nth tree 2) (inc depth)))))

(defn benchmark [rounds]
  (let [values [15 6 18 3 7 17 20 2 4 13 9]]
    (loop [n rounds checksum 0]
      (if (> n 0)
        (let [tree (build-tree values)]
          (recur (dec n)
                 (+ checksum
                    (depth-checksum tree 1)
                    (* 100 (tree-search tree 13))
                    (* 10 (tree-search tree 19)))))
        checksum))))

(defn -main [] (println (benchmark 6000)))
(-main)
