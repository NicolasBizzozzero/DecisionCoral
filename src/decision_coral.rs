/// https://facility9.com/2016/05/writing-documentation-in-rust/
/// https://scikit-learn.org/stable/modules/generated/sklearn.tree.DecisionTreeClassifier.html
/// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
/// https://fr.wikipedia.org/wiki/Arbre_de_décision_(apprentissage)



struct DecisionCoral {
    criterion: String,
    max_depth: u64,
    min_samples_split: u64,
    min_samples_leaf: u64,
    seed: u64,

    tree: , // TODO

    _n_classes: u64,
    _n_features: u64,
    _fitted: bool
}


fn fit(&mut DecisionCoral, X, y) {

}


fn predict(&mut DecisionCoral, X) {

}


fn predict_proba(&mut DecisionCoral, X) {

}


fn predict_log_proba(&mut DecisionCoral, X) {

}


fn score(&mut DecisionCoral, X, y) {

}
