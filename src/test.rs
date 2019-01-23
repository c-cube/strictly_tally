use super::*;

#[test]
fn test_sheet1() {
    let s =
        r"name, j1, j2, j3, j4, j5
        Jim Bob, 1, 4, 2, 2, 5
        Freddy Lou, 3, 1, 3, 4, 2
        Mary Sue, 4, 3, 5, 3, 1
        Bobby Joe, 2, 5, 1, 5, 3
        Candy Jane, 5, 2, 4, 1, 4";

    let mut sheet = Sheet::from_str(s).unwrap();

    assert_eq!(sheet.n_judges(), 5);
    assert_eq!(sheet.n_competitors(), 5);

    let rank = sheet.rank();

    assert_eq!(vec!["Jim Bob", "Freddy Lou", "Bobby Joe", "Mary Sue", "Candy Jane"],
               rank.iter()
               .map(|c| c.as_one().unwrap().name.clone()).collect::<Vec<_>>());
}

#[test]
fn test_sheet2() {
    let s =
        r"name, j1, j2, j3, j4, j5
        Jim Bob, 1, 4, 2, 2, 5
        Freddy Lou, 3, 1, 3, 4, 2
        Mary Sue, 4, 2, 5, 3, 1
        Bobby Joe, 2, 5, 1, 5, 3
        Candy Jane, 5, 2, 4, 1, 4";

    let mut sheet = Sheet::from_str(s).unwrap();

    assert_eq!(sheet.n_judges(), 5);
    assert_eq!(sheet.n_competitors(), 5);

    let rank = sheet.rank();

    assert_eq!(vec!["Jim Bob", "Freddy Lou", "Mary Sue", "Bobby Joe", "Candy Jane"],
               rank.iter()
               .map(|c| c.as_one().unwrap().name.clone()).collect::<Vec<_>>());
}
