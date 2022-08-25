use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MenuWrapper {
    pub menu: Vec<MenuList>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MenuList {
    pub menu_label: String,
    pub menu_list: Vec<MenuItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct MenuItem {
    pub label: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_list: Option<Vec<MenuItem>>,
}

#[test]
fn test() {
    let menu_sub_list = vec![
        MenuItem {
            label: "Members".to_string(),
            name: "members".to_string(),
            menu_list: None,
        },
        MenuItem {
            label: "Plugins".to_string(),
            name: "plugins".to_string(),
            menu_list: None,
        },
        MenuItem {
            label: "Add a member".to_string(),
            name: "add_a_member".to_string(),
            menu_list: None,
        },
    ];

    let menu_list_general = MenuList {
        menu_label: "General".to_string(),
        menu_list: vec![
            MenuItem {
                label: "Dashboard".to_string(),
                name: "dashboard".to_string(),
                menu_list: None,
            },
            MenuItem {
                label: "Customers".to_string(),
                name: "customers".to_string(),
                menu_list: None,
            },
        ],
    };

    let menu_list_administration = MenuList {
        menu_label: "Administration".to_string(),
        menu_list: vec![
            MenuItem {
                label: "Team Settings".to_string(),
                name: "team_settings".to_string(),
                menu_list: None,
            },
            MenuItem {
                label: "Manage Your Team".to_string(),
                name: "manage_your_team".to_string(),
                menu_list: Some(menu_sub_list),
            },
            MenuItem {
                label: "Invitations".to_string(),
                name: "invitations".to_string(),
                menu_list: None,
            },
            MenuItem {
                label: "Cloud Storage Environment Settings".to_string(),
                name: "cloud_storage_environment_settings".to_string(),
                menu_list: None,
            },
            MenuItem {
                label: "Authentication".to_string(),
                name: "authentication".to_string(),
                menu_list: None,
            },
        ],
    };

    let menu_list_transactions = MenuList {
        menu_label: "Transactions".to_string(),
        menu_list: vec![
            MenuItem {
                label: "Payments".to_string(),
                name: "payments".to_string(),
                menu_list: None,
            },
            MenuItem {
                label: "Transfers".to_string(),
                name: "transfers".to_string(),
                menu_list: None,
            },
            MenuItem {
                label: "Balance".to_string(),
                name: "balance".to_string(),
                menu_list: None,
            },
        ],
    };

    let menu = MenuWrapper {
        menu: vec![
            menu_list_general,
            menu_list_administration,
            menu_list_transactions,
        ],
    };

    let json_str = serde_json::to_string_pretty(&menu);
    println!("{}", json_str.unwrap());
}
