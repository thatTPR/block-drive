pub(crate) struct Path {
    pub nodes: Vec<String>, // Determined by string split  on ./
}
pub fn pathSplit(path: String) -> Path {
    return Path::new(path.rsplit('/').collect());
}
impl Path {
    pub fn new(path: String) -> Self {
        Self {
            nodes: pathSplit(&path),
        }
    }
    pub fn parent(&self) {
        return self.nodes.pop().pop();
    }
}
// TODO maybe child using bitmap to figure out system that manages it. Redesign might be necessarry
