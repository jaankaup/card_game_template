use std::fmt;

pub struct ContainerStyle {

    display: contents;
    position: relative;
    color: red;
    background-color: rgb(235,235, 255);
    position: relative;
    align-items: left;
    height: 100%;
    width: 100%;
    margin: 0;
    padding: 0;
}

impl fmt::Display for ContainerStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
