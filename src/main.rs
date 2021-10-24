mod common_concept;
mod enums;
mod functions;
mod ownership;
mod project_manager;
mod structs;
mod collections;

fn main() {
    // common_concept::basic_syntax();
    // common_concept::guess_game();
    //
    // functions::function_test();
    //
    // ownership::ownership_test();
    // structs::structs_test();
    enums::enum_test();
    project_manager::eat_at_restaurant();
    project_manager::front_of_house::hosting::seat_at_table();
}
