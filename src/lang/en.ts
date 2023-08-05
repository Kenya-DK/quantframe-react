export const en = {
  components: {
    forms: {
    },
    searchItemField: {
      title: "Search item",
      description: "Search for an item",
      placeholder: "Search item",
      no_results: "No results",
    },
    inventory: {
      price: "Price",
      price_description: "Price per unit",
      quantity: "Quantity",
      quantity_description: "Quantity of items",
      rank: "Mod Rank",
      rank_description: "Rank of the mod",
      buttons: {
        buy: "Buy",
        sell: "Sell",
      },
      datagrid: {
        columns: {
          name: "Name",
          price: "Price",
          listed_price: "Listed Price",
          owned: "Owned",
          actions: {
            title: "Actions",
            sell: "Sell",
            delete: {
              title: "Delete",
              message: "Are you sure you want to delete this item?",
              buttons: {
                confirm: "Delete",
                cancel: "Cancel",
              }
            }
          },
        }
      }
    },
    modals: {
      prompt: {
        confirmLabel: "Confirm",
        cancelLabel: "Cancel",
      }
    },
  },
  context: {
    wisper: {
      title: "Wisper",
      message: "Wisper {{name}}",
    },
  },
  layout: {
    header: {
      title: "QuantFrame",
      profile: {
        logout: "Log Out",
      }
    },
    navigation: {
      home: "Home",
    },
  },
  pages: {
    home: {
    },
    auth: {
      login: {
        title: "Warframe Market - Login",
        email: "Email",
        password: "Password",
        remember_me: "Remember me",
        submit: "Login",
      },
    },
  },
  success: {
    auth: {
      login_title: "Login success",
      login_message: "Welcome back {{name}}",
      logout_title: "Logout success",
      logout_message: "You have been logged out successfully",
      invite_title: "Invite sent",
      invite_message: "An invite has been sent to {{email}}",
    },
    order: {
      order_title: "Order",
      not_found: "Order not found",
      order_updated: "Order updated successfully #{{id}}",
      order_updated_successfully: "Order updated successfully",
      orders_refresh: "Orders refreshed successfully #{{count}}",
      orders_refresh_successfully: "Orders refreshed successfully",
      printing: {
        title: "Printing",
        message: "Printing order {{id}}",
      },
    },
    invoicing: {
      invoicing_title: "Invoicing",
      order_added: "Order added successfully {{id}}",
    },
    user: {
      updated: {
        title: "User updated",
        message: "User {{name}} was updated successfully",
      },
      created: {
        title: "User created",
        message: "User {{name}} was created successfully",
      },
      profile: {
        updated: {
          title: "Your profile updated",
          message: "Your profile was updated successfully",
        }
      }
    },
    warehouse: {
      updated: {
        title: "Warehouse updated",
        message: "Warehouse {{name}} was updated successfully",
      },
      created: {
        title: "Warehouse created",
      },
    }
  },
  error: {
    invalid_email: "Invalid email",
    auth: {
      login_title: "Login Failed",
      logout_title: "Logout Failed",
      invalidCredentials: "The email/password combination is invalid",
      user_inactive: "The user is inactive please contact an administrator",
      password_invalid: "Password should include at least 6 characters",
    },
    order: {
      order_title: "Order",
      not_found: "Order not found",
    },
    invoicing: {
      invoicing_title: "Invoicing",
      order_already_scanned: "Order {{id}} already scanned",
    },
    user: {
      updated: {
        title: "User updated failed",
      },
      created: {
        title: "User created failed",
      },
      profile: {
        updated: {
          title: "Your profile updated failed",
        }
      }
    },
    warehouse: {
      updated: {
        title: "Warehouse updated failed",
      },
      created: {
        title: "Warehouse created failed",
      },
    },
    printing: {
      title: "Printing failed",
    }
  }
};
