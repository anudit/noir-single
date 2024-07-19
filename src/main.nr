fn main(x: Field, y: pub Field) {
    assert(x != y);
}

#[test]
fn test_add() {
    // assert(main(2,2));
    main(0,1);
    main(1,0);
}
