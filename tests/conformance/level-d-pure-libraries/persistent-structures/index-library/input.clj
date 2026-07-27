(ns d.index-library)
(defn build-index [pairs]
(reduce (fn [index pair]
(assoc index (first pair) (second pair)))
{}
pairs))
(defn -main []
(let [index (build-index [[:a 10] [:b 20] [:c 30]])]
(println (count index) (get index :b) (contains? index :c)
(count (dissoc index :a)))))
(-main)
