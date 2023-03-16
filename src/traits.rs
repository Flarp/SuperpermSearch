pub trait Searchable: Sized + Ord + Clone {
    fn generate_successors(&self) -> Vec<Self>;

    fn f(&self) -> u16;

    fn heuristic(&self) -> u16;
}