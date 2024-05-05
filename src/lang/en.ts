
export const en = {
  months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
  enums: {
    user_status: {
      online: "Online",
      ingame: "Ingame",
      invisible: "Offline",
    },
    stock_mode: {
      all: "All",
      riven: "Riven",
      item: "Item",
    },
    order_mode: {
      buy: "Buy",
      sell: "Sell",
      both: "Both",
    },
    stock_status: {
      pending: "Pending",
      live: "Live",
      to_low_profit: "To Low Profit",
      no_sellers: "No Sellers",
      no_buyers: "No Buyers",
      inactive: "Inactive",
      sma_limit: "SMA Limit",
      order_limit: "Order Limit",
      overpriced: "Overpriced",
      underpriced: "Underpriced",
      details: {
        pending: "Pending TODO",
        live: "Live TODO",
        to_low_profit: "To Low Profit TODO",
        no_sellers: "No Sellers TODO",
        no_buyers: "No Buyers TODO",
        inactive: "Inactive TODO",
        sma_limit: "SMA Limit TODO",
        order_limit: "Order Limit TODO",
        overpriced: "Overpriced TODO",
        underpriced: "Underpriced TODO",
      }
    },
  },
  components: {
    modals: {
      base: {
        buttons: {
          confirm: "Confirm",
          cancel: "Cancel",
        },
      },
    },
    forms: {
      settings: {
        tabs: {
          general: {
            title: "General",
          },
          live_trading: {
            title: "Live Trading",
            fields: {
              volume_threshold: {
                label: "Volume Threshold",
                placeholder: "Volume Threshold",
                error: "Invalid volume threshold",
                tooltip: "The minimum volume per trade.",
              },
              range_threshold: {
                label: "Range Threshold",
                placeholder: "Range Threshold",
                error: "Invalid range threshold",
                tooltip: "The minimum range between the highest price and the lowest price.",
              },
              avg_price_cap: {
                label: "Average Price Cap",
                placeholder: "Average Price Cap",
                error: "Invalid average price cap",
                tooltip: "The maximum average price cap.",
              },
              max_total_price_cap: {
                label: "Max Total Price Cap",
                placeholder: "Max Total Price Cap",
                error: "Invalid max total price cap",
                tooltip: "This parameter specifies the maximum platinum total price cap for the all buy orders.",
              },
              price_shift_threshold: {
                label: "Price Shift Threshold",
                placeholder: "Price Shift Threshold",
                error: "Invalid price shift threshold",
                tooltip: "The minimum price shift threshold for the last 7 days.",
              },
              min_sma: {
                label: "Min SMA",
                placeholder: "Min SMA",
                error: "Invalid min SMA",
                tooltip: "How low the price can go below the SMA.",
              },
              item_min_profit: {
                label: "Min Profit",
                placeholder: "Min Profit",
                error: "Invalid min profit",
                tooltip: "The minimum profit",
              },
              auto_delete: {
                label: "Auto Delete",
                error: "Invalid auto delete",
                tooltip: "Automatically delete stock items",
              },
              stock_mode: {
                label: "Stock Mode",
                description: {
                  all: "Will go through all",
                  riven: "Will go through only rivens",
                  item: "Will go through only items",
                },
                placeholder: "Stock Mode",
                error: "Invalid stock mode",
              },
              order_mode: {
                label: "Order Mode",
                description: {
                  buy: "Will only buy",
                  sell: "Will only sell",
                  both: "Will buy and sell",
                },
                placeholder: "Order Mode",
                error: "Invalid order mode",
              },
              report_to_wfm: {
                label: "Report to Warframe Market",
                error: "Invalid report to Warframe Market",
                tooltip: "Will add a transaction to Warframe Market",
              },
              auto_trade: {
                label: "Auto Trade (WIP)",
                error: "Invalid auto trade",
                tooltip: "Automatically add/sell stock if true",
              },
              strict_whitelist: {
                label: "Strict Whitelist",
                error: "Invalid strict whitelist",
                tooltip: "Only trade items in the whitelist",
              },
              blacklist: {
                description: "The blacklist is a list of items that will not be ignored by the bot. (Sell/Buy)",
                left_title: "Available Items",
                right_title: "Blacklisted Items",
              },
              whitelist: {
                description: "The whitelist is a list of items that will buy no matter the profit",
                left_title: "Available Items",
                right_title: "Whitelisted Items",
              },
              riven_min_profit: {
                label: "Min Profit",
                placeholder: "Min Profit",
                error: "Invalid min profit",
                tooltip: "The minimum profit",
              },
              limit_to: {
                label: "Average Price Cap",
                placeholder: "5",
                error: "Invalid limit to",
                tooltip: "Will use use the first {{count}} rivens to calculate the average price",
              },
              threshold_percentage: {
                label: "Minimum Price Shift",
                placeholder: "0",
                error: "Invalid threshold percentage",
                tooltip: "Ignore rivens that have less than {{value}}% price shift from the most expensive riven",
              },
            },
            tabs: {
              item: "Item Settings",
              riven: "Riven Settings",
            },
            buttons: {
              save: {
                label: "Save",
              },
              blacklist: {
                label: "Blacklist",
              },
              whitelist: {
                label: "Whitelist",
              },
              go_back: {
                label: "Go Back",
              },
            },
          },
          notification: {
            title: "Notification",
          },
        }
      },
      update_stock_riven: {
        fields: {
          minimum_price: {
            label: "Minimum Price",
            placeholder: "Minimum Price",
            description: "Set 0 for auto price",
            error: "Invalid minimum price",
          },
          use_hidden: {
            label: "Use Hidden",
          },
          is_hidden: {
            label: "Is Hidden",
          },
        },
        buttons: {
          submit: "Update",
        },
      },
      create_stock_item: {
        fields: {
          quantity: {
            label: "Quantity",
            placeholder: "Quantity",
            error: "Invalid quantity",
          },
          bought: {
            label: "Bought",
            placeholder: "Bought",
            error: "Invalid bought",
          }
        },
        buttons: {
          add: {
            tooltip: {
              description_with_report: "Add item to stock and report to Warframe Market",
              description_without_report: "Add item to stock",
            }
          }
        },
      },
      riven_filter: {
        fields: {
          enabled: {
            label: "Enabled",
          },
          similarity: {
            label: "Similarity",
          },
          rank: {
            label: "Rank",
          },
          mastery_rank: {
            label: "Mastery Rank",
          },
          required_negative: {
            label: "Required Negative",
          },
          re_rolls: {
            label: "Re-Rolls",
          },
        },
        buttons: {
          save: {
            label: "Save",
          },
        }
      },
      notification: {
        fields: {
          title: {
            label: "Title",
            placeholder: "Title",
            error: "Invalid title",
          },
          content: {
            label: "Content",
            placeholder: "Content",
            error: "Invalid content",
          },
          webhook: {
            label: "Webhook",
            placeholder: "Webhook",
            description: "Discord webhook URL",
            error: "Invalid webhook",
          },
          user_ids: {
            label: "User IDs",
            placeholder: "User IDs",
            description: "Discord user's (Numbers) separated by comma",
            error: "Invalid user IDs",
          }
        },
        buttons: {
          system: {
            tooltip: "System Notification",
          },
          save: {
            label: "Save",
          },
          discord: {
            tooltip: "Discord Notification",
          },
        }
      },
      log_in: {
        title: "Warframe Market - Login",
        register: "Don't have an account? Register",
        fields: {
          email: {
            label: "Email",
            placeholder: "Email",
            error: "Invalid email",
          },
          password: {
            label: "Password",
            placeholder: "Password",
            error: "Password should include at least 6 characters",
          }
        },
        buttons: {
          submit: "Log In",
        },
      }
    },
    stock_item_info: {
      fields: {
        created_at: "Created At",
        updated_at: "Updated At",
        status: "Status",
        bought: "Bought",
        minimum_price: "Minimum Price",
        owned: "Owned",
        listed: "Listed Prices History",
      },
      buttons: {
        wfm: "Warframe Market",
        wiki: "Wiki",
      }
    },
    stock_riven_info: {
      fields: {
        created_at: "Created At",
        updated_at: "Updated At",
        status: "Status",
        bought: "Bought",
        minimum_price: "Minimum Price",
        listed: "Listed Prices History",
      }
    },
    riven_attribute: {
      effect: "{{value}} {{name}}",
    },
    tradableItem_list: {
      fields: {
        trade_tax: {
          label: "Trade Tax Range {{min}} - {{max}}",
          placeholder: "Trade Tax",
        },
        mr_requirement: {
          label: "MR Requirement Range {{min}} - {{max}}",
          placeholder: "MR Requirement",
        },
        tags: {
          label: "Tags",
          placeholder: "Select tags...",
          options: {
            prime: "Prime Parts",
            set: "Set",
            arcane_enhancement: "Arcane",
            tax_1m: "Tax 1M",
            tax_2m: "Tax 2M",
          },
        }
      },
      datatable: {
        columns: {
          name: "Name",
          trade_tax: "Trade Tax",
          mr_requirement: "MR Requirement",
        }
      },
      buttons: {
        add_all: {
          tooltip: "Add all items",
        },
      }
    },
    searchfield: {
      label: "Search",
      placeholder: "Search...",
      buttons: {
        filter: {
          tooltip: "Filter",
        },
        search: {
          tooltip: "Search",
        },
        create: {
          tooltip: "Create",
        },
      }
    },
    select_tradable_item: {
      fields: {
        item: {
          label: "Item",
          placeholder: "Select item...",
        },
        variant: {
          label: "Variant",
          placeholder: "Select variant...",
        },
        rank: {
          label: "Rank",
          placeholder: "Select rank...",
        },
        cyan_stars: {
          label: "Cyan Stars",
          placeholder: "Select cyan stars...",
        },
        amber_stars: {
          label: "Amber Stars",
          placeholder: "Select amber stars...",
        },
      }
    },
    user_menu: {
      items: {
        app_label: "Application",
        settings: "Settings",
        logout: "Logout",
      },
      errors: {
        logout: {
          title: "Logout Error",
          message: "An error occurred while trying to logout.",
        },
        update_settings: {
          title: "Update Settings Error",
          message: "An error occurred while trying to update settings.",
        }
      },
      success: {
        logout: {
          title: "Logout Success",
          message: "You have been successfully logged out.",
        },
        update_settings: {
          title: "Update Settings Success",
          message: "Settings have been successfully updated.",
        }
      },
    },
    clock: {
      gmt: "GMT: <blue>{{time}}</blue>",
      time_until_midnight: "Time until midnight GMT: <blue>{{time}}</blue>",
    },
    layout: {
      log_in: {
        navbar: {
          home: "Home",
          live_trading: "Live Trading",
          chats: "Chats",
          statistics: "Statistics",
          warframe_market: "Warframe Market",
          debug: "Debug",
        },
      },
    },
    live_trading_control: {
      buttons: {
        start: "Start Live Trading",
        stop: "Stop Live Trading",
      },
      item: {
        stating: "Starting Item Trading",
        deleting_orders: "Deleting Orders {{current}}/{{total}}",
        not_in_inventory: "Item <red>{{name}}</red> not in inventory deleting.",
        low_profit_delete: "Deleting Item <red>{{name}}</red> low profit",
        order_limit_reached: "Order limit reached for <red>{{name}}</red>",
        knapsack_delete: "Delete Item <red>{{name}}</red>",
        underpriced_delete: "Delete Underpriced Item <red>{{name}}</red>",
        created: "Created Buy Order for <blue>{{name}}</blue> at <blue>{{price}}</blue> platinum potential profit <blue>{{profit}}</blue> ",
        checking_item: "Checking Item <blue>{{name}}</blue> <blue>{{current}}</blue>/<blue>{{total}}</blue>",
      },
      riven: {
        stating: "Starting Riven Trading",
        searching_riven: "Searching Riven {{weapon_name}} {{mod_name}}</blue> <blue>{{current}}</blue>/<blue>{{total}}</blue>",
        riven_created: "Created Riven <blue>{{weapon_name}} {{mod_name}}</blue> at <blue>{{price}}</blue> platinum potential profit <blue>{{profit}}</blue>",
      },
    },
    riven_filter_attribute: {
      fields: {
        is_required: {
          tooltip: "Is Required",
        },
      },
    }
  },
  context: {
    app: {
      new_update: {
        title: "New Update Available {{version}}",
        message: "A new update is available. Click the button below to install the update.",
        buttons: {
          install: "Install",
          read_more: "Read more"
        },
      },
      loading_events: {
        cache: "Loading cache...",
        validation: "Validating cache...",
        stock_items: "Loading stock items...",
        stock_rivens: "Loading stock rivens...",
        transactions: "Loading transactions...",
        user_orders: "Loading warframe market orders...",
        user_auctions: "Loading warframe market auctions...",
        user_chats: "Loading warframe market chats...",
        check_updates: "Checking for updates...",
      }
    }
  },
  sockets: {
    qf_socket: {
      events: {
        OnAddRivenAlecaFrame: {
          errors: {
            title: "Add Riven Error",
            weapon_not_found: "Weapon not found {{name}}",
            attribute_not_found: "Attribute not found {{name}}",
          },
          success: {
            title: "Add Riven Success",
            message: "Riven {{name}} added successfully",
          }
        }
      }
    },
  },
  pages: {
    home: {
      tooltips: {
        bar_chart: {
          footer: {
            expense: "Total expenses",
            revenue: "Total revenue",
            profit: "Total profit",
            trades: "Number of trades",
            sales: "Number of sales",
            purchases: "Number of purchases",
          }
        }
      },
      cards: {
        total: {
          title: "Total Profit",
          footer: "Sales: <blue>{{sales}}</blue> | Purchases: <blue>{{purchases}}</blue> | <trade/> <blue>{{quantity}}</blue> | Profit Margin: <blue>{{profit_margin}}</blue>%",
          bar_chart: {
            title: "Total Profit",
            datasets: {
              this_year: "This Year",
              last_year: "Last Year",
            },
            footers: {
              profit: "<expenseIco/> <blue>{{expense}}</blue> | <revenueIco/> <blue>{{revenue}}</blue> | <profitIco/> <blue>{{profit}}</blue>",
              trades: "<purchaseIco/> <blue>{{purchases}}</blue> | <saleIco/> <blue>{{sales}}</blue> | <tradeIco/> <blue>{{trades}}</blue>",
            },
          }
        },
        today: {
          title: "Today's Profit",
          footer: "Sales: <blue>{{sales}}</blue> | Purchases: <blue>{{purchases}}</blue> | <trade/> <blue>{{quantity}}</blue> | Profit Margin: <blue>{{profit_margin}}</blue>%",
          bar_chart: {
            title: "Today's Profit",
            datasets: {
              profit: "Profit",
            },
            footers: {
              profit: "<expenseIco/> <blue>{{expense}}</blue> | <revenueIco/> <blue>{{revenue}}</blue> | <profitIco/> <blue>{{profit}}</blue>",
              trades: "<purchaseIco/> <blue>{{purchases}}</blue> | <saleIco/> <blue>{{sales}}</blue> | <tradeIco/> <blue>{{trades}}</blue>",
            },
          }
        },
        recent_days: {
          bar_chart: {
            title: "Last {{days}} days",
            datasets: {
              profit: "Profit",
            },
            footers: {
              profit: "<expenseIco/> <blue>{{expense}}</blue> | <revenueIco/> <blue>{{revenue}}</blue> | <profitIco/> <blue>{{profit}}</blue>",
              trades: "<purchaseIco/> <blue>{{purchases}}</blue> | <saleIco/> <blue>{{sales}}</blue> | <tradeIco/> <blue>{{trades}}</blue>",
            },
          }
        },
        best_seller: {
          title: "Best Seller Profit",
          footer: "Name: <blue>{{name}}</blue> | S: <blue>{{sales}}</blue> | P: <blue>{{purchases}}</blue> | <trade/> <blue>{{quantity}}</blue> | PM: <blue>{{profit_margin}}</blue>%",
          by_category: {
            datatable: {
              columns: {
                name: "Name",
                revenue: "Revenue",
                expense: "Expense",
                profit: "Profit",
                profit_margin: "Profit Margin",
              }
            }
          },
        },
        last_transaction: {
          title: "Last Transaction",
          info_box: {
            purchase: "Purchase {{count}}",
            sale: "Sale {{count}}",
          }
        },
      }
    },
    liveTrading: {
      segments: {
        bought: "Bought",
        listed: "Listed",
        profit: "Profit",
      },
      datatable: {
        columns: {
          name: {
            title: "Name",
            value: "{{name}} <blue>{{sub_type}}</blue>",
          },
          bought: "Bought",
          minimum_price: {
            title: "Minimum Price",
            btn: {
              edit: {
                tooltip: "Set minimum price"
              }
            }
          },
          list_price: "List Price",
          actions: {
            title: "Actions",
            buttons: {
              sell_manual: {
                tooltip: "Sell manually",
              },
              sell_auto: {
                tooltip: "Sell as listed price",
              },
              hide: {
                enabled_tooltip: "Hide",
                disabled_tooltip: "Unhide",
              },
              info: {
                tooltip: "Show Info",
              },
              delete: {
                tooltip: "Delete",
              },
            }
          },
        }
      },
      prompts: {
        minimum_price: {
          title: "Minimum Price",
          fields: {
            minimum_price: {
              label: "Minimum Price",
              description: "Set 0 for auto price"
            }
          },
        },
        sell: {
          title: "Sold Price",
          fields: {
            sell: {
              label: "Price",
            }
          },
        },
      },
      notifications: {
        copied: {
          title: "Copied",
        },
      },
      tabs: {
        item: {
          title: "Stock Items",
          datatable: {
            columns: {
              item_name: "Name",
              quantity: "Quantity",
              owned: "Owned",
            }
          },
          errors: {
            create_stock: {
              title: "Create Stock Error",
              message: "An error occurred while trying to create stock.",
            },
            update_stock: {
              title: "Update Stock Error",
              message: "An error occurred while trying to update stock.",
            },
            sell_stock: {
              title: "Sell Stock Error",
              message: "An error occurred while trying to sell stock.",
            },
            delete_stock: {
              title: "Delete Stock Error",
              message: "An error occurred while trying to delete stock.",
            }
          },
          success: {
            create_stock: {
              title: "Create Stock Success",
              message: "Stock item {{name}} has been successfully created.",
            },
            update_stock: {
              title: "Update Stock Success",
              message: "Stock item {{name}} has been successfully updated.",
            },
            sell_stock: {
              title: "Sell Stock Success",
              message: "Stock item {{name}} has been successfully sold.",
            },
            delete_stock: {
              title: "Delete Stock Success",
              message: "Stock item has been successfully deleted.",
            }
          }
        },
        riven: {
          title: "Stock Rivens",
          datatable: {
            columns: {
              mastery_rank: "MR",
              attributes: "Attributes",
              re_rolls: "Re-Rolls",
              actions: {
                buttons: {
                  filter: {
                    tooltip: "Edit Filter",
                  },
                }
              }
            }
          },
          prompts: {
            update_bulk: {
              title: "Update Bulk",
            },
            update_filter: {
              title: "Update Filter",
            },
          },
          buttons: {
            update_bulk: {
              tooltip: "Update Bulk",
            },
            delete_bulk: {
              tooltip: "Delete Bulk",
            },
            wts: {
              tooltip: "Create WTS Message",
            },
            selection: {
              tooltip: "Create Selection Message",
            },
          },

          errors: {
            update_stock: {
              title: "Update Stock Error",
              message: "An error occurred while trying to update stock.",
            },
            update_bulk_stock: {
              title: "Update Bulk Stock Error",
              message: "An error occurred while trying to update bulk stock.",
            },
            delete_bulk_stock: {
              title: "Delete Bulk Stock Error",
              message: "An error occurred while trying to delete bulk stock.",
            },
            sell_stock: {
              title: "Sell Stock Error",
              message: "An error occurred while trying to sell stock.",
            },
            delete_stock: {
              title: "Delete Stock Error",
              message: "An error occurred while trying to delete stock.",
            }
          },
          success: {
            update_stock: {
              title: "Update Stock Success",
              message: "Stock riven {{name}} has been successfully updated.",
            },
            update_bulk_stock: {
              title: "Update Bulk Stock Success",
              message: "Stock rivens have been successfully updated.",
            },
            delete_bulk_stock: {
              title: "Delete Bulk Stock Success",
              message: "Stock rivens have been successfully deleted.",
            },
            sell_stock: {
              title: "Sell Stock Success",
              message: "Stock riven {{name}} has been successfully sold.",
            },
            delete_stock: {
              title: "Delete Stock Success",
              message: "Stock riven has been successfully deleted.",
            }
          }
        }
      }
    },
    debug: {
      tabs: {
        transaction: {
          title: "Transactions",
        },
        database: {
          title: "Database",
          cards: {
            reset: {
              title: "Reset Database Table",
              buttons: {
                rest: {
                  title: "Reset",
                }
              },
              errors: {
                rest: {
                  title: "Reset Error",
                  message: "An error occurred while trying to reset.",
                }
              },
              success: {
                rest: {
                  title: "Reset Success",
                  message: "Reset has been successfully completed.",
                }
              },
            },
            migrate: {
              title: "Migrate",
              buttons: {
                migrate: {
                  title: "Migrate",
                }
              },
              errors: {
                migrate: {
                  title: "Migrate Error",
                  message: "An error occurred while trying to migrate.",
                }
              },
              success: {
                migrate: {
                  title: "Migrate Success",
                  message: "Migrate has been successfully completed.",
                }
              },
            },
          }
        }
      },
    },
    auth: {
      errors: {
        login: {
          title: "Login Error",
          message: "An error occurred while trying to login.",
        }
      },
      success: {
        login: {
          title: "Login Success",
          message: "Welcome back! {{name}}",
        }
      }
    }
  },
}
