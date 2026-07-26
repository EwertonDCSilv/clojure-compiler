(ns fixture.macros)
(defmacro unless [p x] `(if ~p nil ~x))
