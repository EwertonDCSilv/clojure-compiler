(ns e.external-library (:require [cheshire.core :as json]))
(defn -main [] (println (json/generate-string {:ok true})))
(-main)
