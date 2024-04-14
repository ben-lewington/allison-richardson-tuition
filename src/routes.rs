use once_cell::sync::Lazy;

pub struct Nav<'a, Idx> {
    id: Idx,
    parent_id: Option<Idx>,
    endpoint: &'a str,
    label: &'a str,
}

fn static_nav() -> Vec<Nav<'static, usize>> {
    const TBL: &'static str = r#"
id,parent_id,endpoint,label
0,null,/,Home
1,0,/about,About Us
2,0,/courses,Tuition Services
3,2,/courses/french,French
4,2,/courses/german,German
5,0,/contact,Contact Us
"#;

    TBL.split_terminator('\n')
        .map(|l| {
            let mut row = l.split_terminator(',');
            let id: usize = row.next().unwrap().parse().unwrap();
            let parent_id: Option<usize> = match row.next().unwrap() {
                "null" => None,
                f => Some(f.parse().unwrap()),
            };
            let endpoint = row.next().unwrap();
            let label = row.next().unwrap();

            Nav {
                id,
                parent_id,
                endpoint,
                label,
            }
        })
        .collect()
}

pub static ROUTING: Lazy<Route<'static, usize>> = Lazy::new(|| {
    Route::Nested(
        RouteData::new(0, "/", "Home"),
        vec![
            Route::Simple(RouteData::new(1, "/about", "About Us")),
            Route::Nested(
                RouteData::new(2, "/courses", "Tuition Services"),
                vec![
                    Route::Simple(RouteData::new(4, "/courses/french", "French")),
                    Route::Simple(RouteData::new(5, "/courses/german", "German")),
                ],
            ),
            Route::Simple(RouteData::new(3, "/contact", "Contact Us")),
        ],
    )
});

pub struct RouteData<'a, Idx> {
    pub id: Idx,
    pub route: &'a str,
    pub label: &'a str,
}

pub enum Route<'a, Idx> {
    Simple(RouteData<'a, Idx>),
    Nested(RouteData<'a, Idx>, Vec<Route<'a, Idx>>),
}

impl<'a, Idx> Route<'a, Idx> {
    pub fn home(&self) -> &RouteData<'a, Idx> {
        let Self::Nested(h, _) = self else {
            panic!("only one home page currently supported")
        };
        &h
    }
}

impl<'a, Idx> RouteData<'a, Idx> {
    pub fn new(id: Idx, route: &'a str, label: &'a str) -> Self {
        RouteData { id, route, label }
    }
}
