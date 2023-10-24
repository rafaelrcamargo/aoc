(ns first)

(require '[clojure.string :refer [split split-lines]])
(require '[clojure.core.match :refer [match]])

(defn input [file] (split-lines (slurp file)))
(defn parse [line] (split line #" "))

(defn value [hand]
  (cond
    (or (= hand "A") (= hand "X")) 1
    (or (= hand "B") (= hand "Y")) 2
    (or (= hand "C") (= hand "Z")) 3))

(defn points [hand x] (->> (last hand) (value) (+ x)))

(defn winner [hand]
  (match [(value (first hand)), (value (last hand))]
    ;; Won
    [1, 2] (points hand 6)
    [3, 1] (points hand 6)
    [2, 3] (points hand 6)

    ;; Lose
    [2, 1] (points hand 0)
    [1, 3] (points hand 0)
    [3, 2] (points hand 0)

    ;; How?
    :else 0))

(defn round [hand]
  (if (= (value (first hand)) (value (last hand)))
    (points hand 3) ;; Draw
    (winner hand))) ;; Else

(defn play [line] (-> line parse round))
(defn first-part []
  (->>
   (input "./assets/input.txt")
   (map play)
   (reduce +)))
