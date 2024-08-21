

enum ViewType {
    Percent,
    Pixel,
    ViewportHeight,
    ViewportWidth
}

struct Block {
    pub name: String,
    pub id: u32,
    pub width: u32,
    pub width_type: ViewType,
    pub height: u32,
    pub height_type: ViewType,
    pub parent: Option<*mut Block>,
    pub childrens: Option<Vec<*mut Block>>
}

impl Block {
    fn new (name: String, id: u32, width: u32, width_type: ViewType, height: u32, height_type: ViewType,
            parent: Option<*mut Block>, childrens: Option<Vec<*mut Block>>) -> Block {
        Block {
            name: name,
            id: id,
            width: width,
            width_type: width_type,
            height: height,
            height_type: height_type,
            parent: parent,
            childrens: childrens
        }
    }
}


pub mod rmxl_rider {

    fn readfile (filepath: &str) -> File {

    }
}