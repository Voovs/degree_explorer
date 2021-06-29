use yew::prelude::*;
use yew::services::DialogService;

#[allow(dead_code)]
pub struct Table {
    props: TableProps,
    link: ComponentLink<Self>,
    cells: Vec<Cell>,
}

#[derive(Clone, Properties)]
pub struct TableProps {
    pub size: (u8, u8),
}

pub enum TableMsg {
    AddCell(Cell),
    Clear,
}

impl Component for Table {
    type Properties = TableProps;
    type Message = TableMsg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cells = vec![
            Cell::new("rust lang".to_string() , 300),
            Cell::new("shell opts".to_string(), 200),
            Cell::new("arch linux".to_string(), 100),
            Cell::new("networks".to_string()  , 400),
        ];

        Self { props, link, cells }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TableMsg::AddCell(cell) => {
                self.cells.push(cell);
                true
            }
            TableMsg::Clear => {
                if DialogService::confirm("Clear all courses?") {
                    self.cells.clear();
                    true
                } else { false }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if let (x, 0) = _props.size {
            DialogService::alert(&format!("The table has no size ({}, 0)", x));
        }
        false
    }

    fn view(&self) -> Html {
        let add_course = self.link.callback(|_| {
            TableMsg::AddCell(Cell::new("new course".to_string(), 300))
        });
        let clear_table =  self.link.callback(|_| TableMsg::Clear);

        html! {
            <div class={"table"}>
                <button onclick=add_course>{"Add course"}</button>
                <div>
                    { self.cells.iter().map(|cell| cell.render()).collect::<Html>() }
                </div>
                <button onclick=clear_table>{"Clear"}</button>
            </div>
        }
    }
}


pub struct Cell {
    name: String,
    code: u32,
}


//#[derive(Clone, PartialEq, Properties)]
//pub struct CourseInfo {
//    is_clear: bool,
//    name: String,
//    code: u32,
//}


//pub enum CellMsg {
//    SetCourse(CourseInfo),
//    Clear,
//}


impl Cell {
    pub fn new(name: String, code: u32) -> Self {
        Self { name, code, }
    }


    pub fn render(&self) -> Html {
        html! {
            <div class={"cell"}>
                <span class={"course_name"}>{ &self.name }</span>
                <span class={"course_code"}>{ self.code }</span>
            </div>
        }
    }
}


//impl Component for Cell {
//    type Message = CellMsg;
//    type Properties = CourseInfo;
//
//    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//        Self { props, link }
//    }
//
//    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        match msg {
//            CellMsg::SetCourse(props) => {
//                self.props = props;
//                self.props.is_clear = false;
//            }
//            CellMsg::Clear => self.props.is_clear = true,
//        }
//        true
//    }
//
//    fn change(&mut self, props: Self::Properties) -> ShouldRender {
//        if self.props != props {
//            self.props = props;
//            true
//        } else { false }
//    }
//
//    fn view(&self) -> Html {
//        if !self.props.is_clear {
//            html! {
//                <div class={"cell"}>
//                    <span class={"course_name"}>{ &self.props.name }</span>
//                    <span class={"course_code"}>{ self.props.code }</span>
//                </div>
//            }
//        } else {
//            html! { <div/> }
//        }
//
//    }
//}
