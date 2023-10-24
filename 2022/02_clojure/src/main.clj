(ns main
  (:require [first :refer [first-part]]
            [second :refer [second-part]]))

;; Main
(defn -main []
  (println "1 - First:" (first-part))
  (println "2 - Second:" (second-part)))
