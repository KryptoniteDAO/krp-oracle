use cosmwasm_schema::write_api;

use oracle_pyth::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        // migrate: MigrateMsg
    }
}
