use crate::Grid;

pub struct TestData {
}

impl TestData {
    pub fn simple() -> Grid {
        Grid::from_string("\
            300 002 960\
            140 000 208\
            002 971 000\
            001 400 026\
            204 085 090\
            708 010 003\
            453 000 010\
            010 754 000\
            007 109 604")
    }

    pub fn medium() -> Grid {
        Grid::from_string("\
            002 503 000\
            008 060 000\
            000 020 059\
            000 689 305\
            000 000 080\
            020 005 006\
            007 000 204\
            000 804 000\
            009 700 010")
    }

    pub fn hard() -> Grid {
        // Previous hard puzzle
        Grid::from_string("\
            100 007 090\
            030 020 008\
            009 600 500\
            005 300 900\
            010 080 020\
            600 004 003\
            007 008 000\
            000 030 080\
            020 400 001")
    }

    pub fn expert() -> Grid {
        // Very hard puzzle requiring advanced techniques
        Grid::from_string("\
            000 050 030\
            000 704 000\
            100 300 049\
            000 490 281\
            032 580 900\
            000 000 000\
            965 000 000\
            000 000 000\
            007 200 000")
    }
}
