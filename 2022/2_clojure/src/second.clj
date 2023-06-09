(ns second)

(require '[clojure.string :refer [split split-lines]])

(defn input [file] (split-lines (slurp file)))

(defn parse [line] (split line #" "))

(defn value [hand]
  (cond
    (or (= hand "A") (= hand "X")) 1
    (or (= hand "B") (= hand "Y")) 2
    (or (= hand "C") (= hand "Z")) 3))

(defn points [hand add] (+ (value hand) add))

(defn lose [hand]
  (cond
    (= hand "A") "Z"
    (= hand "B") "X"
    (= hand "C") "Y"))

(defn win [hand]
  (cond
    (= hand "A") "Y"
    (= hand "B") "Z"
    (= hand "C") "X"))

(defn winner [hand]
  (cond
    (= (last hand) "X") (points (lose (first hand)) 0)
    (= (last hand) "Z") (points (win (first hand)) 6)))

(defn round [hand]
  (if (= (last hand) "Y")
    (points (first hand) 3) ;; Draw
    (winner hand))) ;; Else

(defn play [line] (-> line parse round))
(defn second-part []
  (->>
   (input "./assets/input.txt")
   (map play)
   (reduce +)))