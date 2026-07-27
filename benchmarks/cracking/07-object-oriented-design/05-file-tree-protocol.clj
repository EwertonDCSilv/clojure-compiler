(ns cracking.objects.file-tree)

(defprotocol NodeSize (node-size [this]))
(defrecord FileNode [byte-count])
(defrecord FolderNode [children])

(extend-type FileNode NodeSize
  (node-size [this] (:byte-count this)))

(extend-type FolderNode NodeSize
  (node-size [this]
    (reduce (fn [total child] (+ total (node-size child)))
            0
            (:children this))))

(defn sample-tree []
  (->FolderNode
    (list (->FileNode 120)
          (->FolderNode (list (->FileNode 80) (->FileNode 40)))
          (->FileNode 300))))

(defn benchmark [rounds]
  (loop [n rounds total 0]
    (if (> n 0)
      (recur (dec n) (+ total (node-size (sample-tree))))
      total)))

(defn -main [] (println (benchmark 3000)))
(-main)
