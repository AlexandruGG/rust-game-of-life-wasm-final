//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate conway_game_of_life_rust_wasm;
use conway_game_of_life_rust_wasm::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_tick() {
    // 0 1 2 3 4 5
    // 0 0 0 0 0 0  0
    // 0 0 X 0 0 0  1
    // 0 0 0 X 0 0  2
    // 0 X X X 0 0  3
    // 0 0 0 0 0 0  4
    // 0 0 0 0 0 0  5
    let mut input_universe = Universe::new();
    input_universe.set_width(6);
    input_universe.set_height(6);
    input_universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);

    // 0 1 2 3 4 5
    // 0 0 0 0 0 0  0
    // 0 0 0 0 0 0  1
    // 0 X 0 X 0 0  2
    // 0 0 X X 0 0  3
    // 0 0 X 0 0 0  4
    // 0 0 0 0 0 0  5
    let mut expected_universe = Universe::new();
    expected_universe.set_width(6);
    expected_universe.set_height(6);
    expected_universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[wasm_bindgen_test]
fn test_toggle_cell() {
    // 0 1 2
    // X 0 0  0
    // 0 X 0  1
    // 0 0 0  2

    let mut input_universe = Universe::new();
    input_universe.set_width(3);
    input_universe.set_height(3);
    input_universe.set_cells(&[(0, 0), (1, 1)]);

    // 0 1 2
    // X 0 0  0
    // 0 0 0  1
    // 0 0 X  2
    let mut expected_universe = Universe::new();
    expected_universe.set_width(3);
    expected_universe.set_height(3);
    expected_universe.set_cells(&[(0, 0), (2, 2)]);

    input_universe.toggle_cell(1, 1);
    input_universe.toggle_cell(2, 2);
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
