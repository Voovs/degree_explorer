use yew::prelude::*;
use yew::services::DialogService;

pub struct Table {
    props: TableProps,
    link: ComponentLink<Self>,
    rows: Vec<TableRow>,
}

#[derive(Clone, Properties)]
pub struct TableProps {
    pub size: (u8, u8),
}

pub enum TableMsg {
    AddCell(usize, RowMsg),
    RemoveCell(usize, RowMsg),
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
        ];

        let rows = vec![TableRow::new(4, Some(cells.clone())); 3];

        Self { props, link, rows }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TableMsg::AddCell(r, msg) => self.rows[r].update(msg),
            TableMsg::RemoveCell(r, msg) => self.rows[r].update(msg),
            TableMsg::Clear => {
                let mut is_update = false;

                if DialogService::confirm("Clear all courses?") {
                    for row in self.rows.iter_mut() {
                        is_update = row.update(RowMsg::Clear) || is_update;
                    }
                }
                is_update
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
            TableMsg::AddCell(2, RowMsg::AddCell(2, Cell::new("new course".to_string(), 300)))
        });

        let clear_table =  self.link.callback(|_| TableMsg::Clear);

        html! {
            <div>
                <button onclick=add_course>{"Add course"}</button>
                    { self.rows.iter().map(|row| row.render()).collect::<Html>() }
                <button onclick=clear_table>{"Clear"}</button>
            </div>
        }
    }
}


/// Row in the table, representing one semester of study
#[derive(Clone)]
pub struct TableRow {
    cells: Vec<Option<Cell>>,
    len: usize,
}

pub enum RowMsg {
    AddCell(usize, Cell),
    RemoveCell(usize),
    Clear,
}

impl TableRow {
    fn new(len: usize, front_cells: Option<Vec<Cell>>) -> Self {
        let mut cells = vec![None; len];

        if let Some(front_cells) = front_cells {
            front_cells.into_iter().enumerate().for_each(|(i, cell)| cells[i] = Some(cell));
        }

        Self { len, cells }
    }

    fn update(&mut self, msg: RowMsg) -> ShouldRender {
        match msg {
            RowMsg::AddCell(i, cell) => {
                if self.cells[i].is_some() {
                    let is_confirm = DialogService::confirm(
                        &format!("Are you sure you want to replace {:?}",
                            self.cells[i].as_ref().unwrap().name)
                    );

                    if is_confirm {
                        self.cells[i].replace(cell);
                    } else {
                        return false
                    }
                } else {
                    self.cells[i].replace(cell);
                }
            }
            RowMsg::RemoveCell(i) => { self.cells[i].take(); },
            RowMsg::Clear => self.cells.fill(None),
        }
        true
    }

    fn render(&self) -> Html {
        html! {
            <div class={ "table-row" }>
                {
                    self.cells.iter().map(|opt_cell| {
                        match opt_cell {
                            Some(cell) => cell.render(),
                            None => Cell::render_blank(),
                        }
                    }).collect::<Html>()
                }
            </div>
        }
    }
}


/// Single course in the table
#[derive(Clone)]
pub struct Cell {
    name: String,
    code: u32,
}

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

    pub fn render_blank() -> Html {
        html!{ <div class={"cell"} /> }
    }
}
