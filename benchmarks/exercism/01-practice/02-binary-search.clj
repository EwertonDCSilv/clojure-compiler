;; Upstream: Exercism Clojure Track, exercise "binary-search".
;; Source: https://github.com/exercism/clojure/blob/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190/exercises/practice/binary-search/.meta/example.clj
;; License: MIT; attribution and local benchmark changes: ../UPSTREAM.md

(ns binary-search)

(defn search-for
  [n coll]
  (loop [low-idx 0
         high-idx (dec (count coll))]
    (if (> low-idx high-idx)
      -1
      (let [mid-index (quot (+ high-idx low-idx) 2)
            mid-item (get coll mid-index)]
        (cond
          (= n mid-item) mid-index
          (> mid-item n) (recur low-idx (dec mid-index))
          :else (recur (inc mid-index) high-idx))))))

(defn benchmark [rounds]
  (let [search-values [0 2 4 6 8 10 12 14 16 18 20 22 24 26 28 30
                       32 34 36 38 40 42 44 46 48 50 52 54 56 58 60 62]]
    (loop [i 0
           checksum 0]
      (if (= i rounds)
        checksum
        (recur (inc i)
               (+ checksum (search-for (mod i 64) search-values)))))))

(defn -main [] (println (benchmark 200000)))

(-main)
