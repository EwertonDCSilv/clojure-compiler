(ns d.tree-library)
(defn node [value left right] {:value value :left left :right right})
(defn total [tree]
(if (nil? tree)
0
(+ (:value tree) (total (:left tree)) (total (:right tree)))))
(defn depth [tree]
(if (nil? tree)
0
(inc (max (depth (:left tree)) (depth (:right tree))))))
(defn -main []
(let [tree (node 5 (node 3 (node 1 nil nil) nil) (node 6 nil nil))]
(println (total tree) (depth tree))))
(-main)
