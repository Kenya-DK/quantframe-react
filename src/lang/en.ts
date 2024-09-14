
export const en = {
  months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
  notifications: {
    add_riven: {
      success: {
        title: "Riven Added",
        message: "Riven {{name}} has been added to the stock.",
      },
      error: {
        title: "Riven Add Error",
        message: "An error occurred in {{component}} at {{backtrace}} see logs for more information.",
      },
    },
    on_trade_event: {
      success: {
        riven: {
          title: "Trade with {{player_name}} complete",
          message: "x{{quantity}} {{item_name}}",
        },
        item: {
          title: "Trade with {{player_name}} complete",
          message: "x{{quantity}} {{item_name}} Order:{{order}} Stock:{{stock}} Transaction:{{transaction}}",
        },
      },
      warning: {
        riven: {
          title: "Trade Warning",
          message: "⚠️ Trade was not processed.",
        },
        item: {
          title: "Trade Warning",
          message: "⚠️ Trade was not processed.",
        },
        created_stock: {
          title: "Trade Warning",
          message: "⚠️ Trade was not processed.",
        }
      },
      error: {
        item: {
          title: "Trade Error",
          message: "An error occurred while trying to trade item {{name}}.",
        },
        riven: {
          title: "Trade Error",
          message: "An error occurred while trying to trade riven {{name}}.",
        },
      },
    },
  },
  enums: {
    transaction_type: {
      purchase: "Purchase",
      sale: "Sale",
      details: {
        purchase: "Purchase",
        sale: "Sale",
      }
    },
    item_type: {
      riven: "Riven",
      item: "Item",
      details: {
        riven: "Riven",
        item: "Item",
      }
    },
    user_status: {
      online: "Online",
      ingame: "Ingame",
      invisible: "Offline",
    },
    order_type: {
      buy: "Buy Order",
      sell: "Sell Order",
      details: {
        buy: "Buy",
        sell: "Sell",
      }
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
                tooltip: "The maximum price cap per buy order.",
              },
              trading_tax_cap: {
                label: "Trading Tax Cap",
                placeholder: "Trading Tax Cap",
                error: "Invalid trading tax cap",
                tooltip: "The maximum credit tax per buy order use -1 for unlimited.",
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
          log: {
            title: "Log",
            buttons: {
              open: {
                label: "Open Log Folder",
              },
              export: {
                label: "Export Logs",
              },
            },
          },
          analytics: {
            title: "Analytics",
            fields: {
              transaction: {
                label: "Transaction",
                tooltip: "If enabled will send transaction events to QF Api",
              },
              stock_item: {
                label: "Stock Item",
                tooltip: "if enabled will send stock item to QF Api",
              },
              stock_riven: {
                label: "Stock Riven",
                tooltip: "if enabled will send stock riven to QF Api",
              },
            },
            buttons: {
              save: {
                label: "Save",
              },
            },
          }
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
      update_stock_item: {
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
      update_transaction: {
        fields: {
          price: {
            label: "Price",
            placeholder: "Price",
            description: "The price of the transaction",
            error: "Invalid price",
          },
          quantity: {
            label: "Quantity",
            placeholder: "Quantity",
            description: "The quantity of the transaction",
            error: "Invalid quantity",
          },
        },
        buttons: {
          submit: "Update",
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
      },
      create_riven_attributes: {
        fields: {
          positive: {
            title: "Positive Attributes",
            add: "Add",
          },
          negative: {
            title: "Negative Attributes",
          },
        },
      },
      create_riven: {
        buttons: {
          submit: {
            label: "Create",
          }
        },
        fields: {
          mastery_rank: {
            label: "Mastery Rank",
            placeholder: "8",
            error: "Invalid mastery rank",
          },
          re_rolls: {
            label: "Re-Rolls",
            placeholder: "0",
            error: "Invalid re-rolls",
          },
          rank: {
            label: "Rank",
            placeholder: "0",
            error: "Invalid rank",
          },
          polarity: {
            label: "Polarity",
            error: "Invalid polarity",
          },
          weapon: {
            label: "Weapon Name",
            placeholder: "Weapon",
            error: "Invalid weapon",
          },
          bought: {
            label: "Bought",
            placeholder: "Bought",
            error: "Invalid bought",
          },
          attributes: {
            label: "Attributes",
            error: "Riven must have at least 2 positive attributes",
          },
          mod_name: {
            label: "Mod Name",
            placeholder: "Mod Name",
            error: "Invalid mod name",
          },
        },
      }
    },
    auction_list_item: {
      weapon_name: "{{weapon}} {{mod_name}}",
      selling_price: "Selling Price: <blue>{{price}}</blue> <plat/>",
      starting_price: "Starting Price: <blue>{{price}}</blue> <plat/>",
      buyout_price: "Buyout Price: <blue>{{price}}</blue> <plat/>",
      top_bid: "Top Bid: <blue>{{bid}}</blue> <plat/>",
      no_bids: "No Bids",
      footer: "Mr: <blue>{{mastery_level}}</blue> Ranks: <blue>{{mod_rank}}</blue> Re-rolls: <blue>{{re_rolls}}</blue> Polarity: <polarity/>",
    },
    stock_item_info: {
      tabs: {
        general: {
          title: "General",
        },
        orders: {
          title: "Orders",
        },
      },
      fields: {
        created_at: "Created At",
        updated_at: "Updated At",
        minimum_price: "Minimum Price",
        moving_avg: "Moving Avg",
        list_price: "List Price",
        profit: "Profit",
        total_sellers: "Total Sellers",
        highest_price: "Highest Price",
        lowest_price: "Lowest Price",
        status: "Status",
        bought: "Bought",
        owned: "Owned",
        listed: "Listed Prices History",
        no_orders: "No orders was found for this item.",
        no_listed: "No previous listed prices",
      },
      buttons: {
        wfm: "Warframe Market",
        wiki: "Wiki",
      }
    },
    order_details: {
      tabs: {
        general: {
          title: "General",
        },
        orders: {
          title: "Orders",
        },
      },
      fields: {
        created_at: "Created At",
        updated_at: "Updated At",
        list_price: "List Price",
        profit: "Profit",
        total_buyers: "Total Buyers",
        highest_price: "Highest Price",
        lowest_price: "Lowest Price",
        listed: "Listed Prices History",
        no_listed: "No previous listed prices",
        no_orders: "No orders was found for this item.",
      },
      buttons: {
        wfm: "Warframe Market",
        wiki: "Wiki",
      }
    },
    order_item: {
      fields: {
        quantity: "<qty/> <blue>{{quantity}}</blue>",
        platinum: "<blue>{{platinum}}</blue> <plat/>",
        mod_rank: "Rank: <blue>{{mod_rank}}</blue>/<blue>{{mod_max_rank}}</blue>",
        subtype: "<blue>{{sub_type}}</blue>",
      },
      notifications: {
        copied: {
          title: "Copied",
          message: "{{message}} has been copied to clipboard.",
        },
      }
    },
    stock_riven_info: {
      tabs: {
        general: {
          title: "General",
        },
        auctions: {
          title: "Auctions",
        },
      },
      fields: {
        created_at: "Created At",
        updated_at: "Updated At",
        status: "Status",
        bought: "Bought",
        minimum_price: "Minimum Price",
        re_rolls: "Re-Rolls",
        list_price: "List Price",
        profit: "Profit",
        total_sellers: "Total Sellers",
        highest_price: "Highest Price",
        lowest_price: "Lowest Price",
        listed: "Listed Prices History",
        no_auctions: "No auctions was found for this riven.",
        no_listed: "No previous listed prices",
        mastery_rank: "Master Rank",
        rank: "Rank",
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
          about: "About",
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
        is_hidden: "Item <red>{{name}}</red> is hidden",
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
        log_parser: "Starting log parser...",
      }
    },
    live_scraper: {
      errors: {
        run: {
          title: "Live Scraper Error",
          message: "An error occurred in component {{component}} at {{backtrace}} see logs for more information.",
        },
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
    about: {
      cards: {
        coffee: {
          title: "Buy me a coffee",
        },
        discord: {
          title: "Discord",
        },
        faq: {
          title: "FAQ",
        },
        guide: {
          title: "Wiki/Guide",
        },
      },
      text: {
        version: "Version: <blue>{{version}}</blue>",
        disclaimer: "Quantframe is a third party app and is not affiliated with Digital Extremes.",
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
        delete: {
          title: "Delete Item's",
          message: "Are you sure you want to delete(s) {{count}}, this action cannot be undone.",
          confirm: "Yes, delete",
          cancel: "No, cancel",
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
          prompts: {
            update_bulk: {
              title: "Update Bulk",
            }
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
            update_bulk_stock: {
              title: "Update Bulk Stock Error",
              message: "An error occurred while trying to update bulk stock.",
            },
            sell_stock: {
              title: "Sell Stock Error",
              message: "An error occurred while trying to sell stock.",
            },
            delete_stock: {
              title: "Delete Stock Error",
              message: "An error occurred while trying to delete stock.",
            },
            delete_bulk_stock: {
              title: "Delete Bulk Stock Error",
              message: "An error occurred while trying to delete bulk stock.",
            },
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
            update_bulk_stock: {
              title: "Update Bulk Stock Success",
              message: "Stock rivens have been successfully updated.",
            },
            sell_stock: {
              title: "Sell Stock Success",
              message: "Stock item {{name}} has been successfully sold.",
            },
            delete_stock: {
              title: "Delete Stock Success",
              message: "Stock item has been successfully deleted.",
            },
            delete_bulk_stock: {
              title: "Delete Bulk Stock Success",
              message: "Stock rivens have been successfully deleted.",
            },
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
            create_riven: {
              title: "Create Riven",
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
            create_riven: {
              tooltip: "Create Riven",
            },
          },
          errors: {
            create_riven: {
              title: "Create Riven Error",
              message: "An error occurred while trying to create riven.",
            },
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
            create_riven: {
              title: "Create Riven Success",
              message: "Riven {{name}} has been successfully created.",
            },
            update_stock: {
              title: "Update Stock Success",
              message: "Stock riven {{name}} has been successfully updated.",
            },
            update_bulk_stock: {
              title: "Update Bulk Stock Success",
              message: "Stock rivens have been successfully updated.",
            },
            sell_stock: {
              title: "Sell Stock Success",
              message: "Stock riven {{name}} has been successfully sold.",
            },
            delete_stock: {
              title: "Delete Stock Success",
              message: "Stock riven has been successfully deleted.",
            },
            delete_bulk_stock: {
              title: "Delete Bulk Stock Success",
              message: "Stock rivens have been successfully deleted.",
            },
          }
        }
      }
    },
    debug: {
      tabs: {
        transaction: {
          title: "Transactions",
          prompts: {
            delete: {
              title: "Delete Transaction",
              message: "Are you sure you want to delete transaction {{name}}, this action cannot be undone.",
              confirm: "Yes, delete",
              cancel: "No, cancel",
            },
            update: {
              title: "Update Transaction",
            },
          },
          datatable: {
            columns: {
              id: {
                title: "ID",
              },
              name: {
                title: "Name",
                value: "{{name}} {{mod_name}} <blue>{{sub_type}}</blue>",
              },
              item_type: {
                title: "Item Type",
              },
              quantity: {
                title: "Quantity",
              },
              price: {
                title: "Price",
              },
              created_at: {
                title: "Created At",
              },
              actions: {
                title: "Actions",
                buttons: {
                  update: {
                    tooltip: "Update",
                  },
                  delete: {
                    tooltip: "Delete",
                  },
                }
              }
            }
          },
          success: {
            update_transaction: {
              title: "Update Transaction Success",
              message: "Transaction {{name}} has been successfully updated.",
            },
            delete_transaction: {
              title: "Delete Transaction Success",
              message: "Transaction has been successfully deleted.",
            },
          },
          errors: {
            update_transaction: {
              title: "Update Transaction Error",
              message: "An error occurred while trying to update transaction.",
            },
            delete_transaction: {
              title: "Delete Transaction Error",
              message: "An error occurred while trying to delete transaction.",
            },
          },
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
            import_algo_trader: {
              title: "Import Algo Trader",
              fields: {
                db_path: {
                  label: "File",
                  placeholder: "Select file...",
                }
              },
              buttons: {
                import: {
                  title: "Import",
                },
                open_file: {
                  tooltip: "Open File",
                }
              },
              errors: {
                import: {
                  title: "Import Error",
                  message: "An error occurred while trying to import.",
                }
              },
              success: {
                import: {
                  title: "Import Success",
                  message: "Import has been successfully completed.",
                }
              },
            },
          }
        }
      },
    },
    auth: {
      progress: {
        logging_in: "Logging In",
        refreshing_orders: "Refreshing Orders",
        refreshing_auctions: "Refreshing Auctions",
        refreshing_chat: "Refreshing Chat",
        refreshing_cache: "Refreshing Cache",
        refreshing_transaction: "Refreshing Transaction",
        refreshing_stock_items: "Refreshing Stock Items",
        refreshing_stock_riven: "Refreshing Stock Rivens",
      },
      errors: {
        login: {
          title: "Login Error",
          email_not_exist: "Email not exist",
          password_invalid: "Password invalid",
          message: "An error occurred while trying to login.",
          banned: "You are banned",
          ban_reason: "<red>Reason: {{reason}}</red>",
        }
      },
      success: {
        login: {
          title: "Login Success",
          message: "Welcome back! {{name}}",
        }
      }
    },
    error: {
      title: "Error in {{component}} component",
      backtrace: "Location: {{backtrace}}",
      cause: "Cause: {{cause}}",
      footer: "If you think this is a bug, please report it to the developer. Thank you.",
    },
    banned: {
      wfm: {
        title: "Account Suspended",
        message: "Unable to connect to Warframe Market, please try again later.",
      },
      qf: {
        title: "Account Suspended",
        reason: "Reason: {{reason}}",
      },
    },
    warframe_market: {
      tabs: {
        orders: {
          title: "Orders",
          buttons: {
            sell_manual: {
              buy: {
                tooltip: "Bought manually",
              },
              sell: {
                tooltip: "Sold manually",
              },
            },
            sell_auto: {
              buy: {
                tooltip: "Bought for listed price",
              },
              sell: {
                tooltip: "Sold for listed price",
              },
            },
            delete: {
              tooltip: "Delete"
            },
            refresh: {
              tooltip: "Refresh"
            },
            delete_all: {
              tooltip: "Delete All"
            },
            info: {
              tooltip: "Show Info"
            },
          },
          prompts: {
            delete: {
              title: "Delete Order",
              message: "Are you sure you want to delete order {{name}}, this action cannot be undone.",
              confirm: "Yes, delete",
              cancel: "No, cancel",
            },
            delete_all: {
              title: "Delete All Orders",
              message: "Are you sure you want to delete all orders, this action cannot be undone.",
              confirm: "Yes, delete",
              cancel: "No, cancel",
            },
            sell: {
              title: "Manual Sell",
              field: {
                label: "Sold For",
              },
            },
            buy: {
              title: "Manual Buy",
              field: {
                label: "Bought For",
              },
            },
          },
          success: {
            create_stock: {
              title: "Create Stock Success",
              message: "Stock item {{name}} has been successfully created.",
            },
            sell_stock: {
              title: "Sell Stock Success",
              message: "Stock item {{name}} has been successfully sold.",
            },
            refresh: {
              title: "Refresh Success",
              message: "Total {{count}} orders have been successfully refreshed.",
            },
            delete: {
              title: "Delete Success",
              message: "Order has been successfully deleted.",
            },
            delete_all: {
              title: "Delete All Success",
              message: "All orders have been successfully deleted.",
            },
          },
          errors: {
            create_stock: {
              title: "Create Stock Error",
              message: "An error occurred while trying to create stock.",
            },
            sell_stock: {
              title: "Sell Stock Error",
              message: "An error occurred while trying to sell stock.",
            },
            refresh: {
              title: "Refresh Error",
              message: "An error occurred while trying to refresh orders.",
            },
            delete: {
              title: "Delete Error",
              message: "An error occurred while trying to delete order.",
            },
            delete_all: {
              title: "Delete All Error",
              message: "An error occurred while trying to delete all orders.",
            },
          },
        },
        auctions: {
          title: "Auctions",
          buttons: {
            refresh: {
              tooltip: "Refresh"
            },
            delete_all: {
              tooltip: "Delete All"
            },
            delete: {
              tooltip: "Delete"
            },
            import: {
              tooltip: "Import auction to stock"
            },
          },
          prompts: {
            import_riven: {
              title: "Import riven to stock",
              bought: {
                label: "Bought For",
              },
            },
            delete: {
              title: "Delete Auction",
              message: "Are you sure you want to delete auction, this action cannot be undone.",
              confirm: "Yes, delete",
              cancel: "No, cancel",
            },
            delete_all: {
              title: "Delete All Auctions",
              message: "Are you sure you want to delete all auctions, this action cannot be undone.",
              confirm: "Yes, delete",
              cancel: "No, cancel",
            },
          },
          success: {
            import_riven: {
              title: "Import Success",
              message: "Riven has been successfully imported to stock.",
            },
            refresh: {
              title: "Refresh Success",
              message: "Total {{count}} auctions have been successfully refreshed.",
            },
            delete: {
              title: "Delete Success",
              message: "Auction has been successfully deleted.",
            },
            delete_all: {
              title: "Delete All Success",
              message: "All orders have been successfully deleted.",
            }
          },
          errors: {
            refresh: {
              title: "Refresh Error",
              message: "An error occurred while trying to refresh auctions.",
            },
            import_riven: {
              title: "Import Error",
              message: "An error occurred while trying to import riven.",
            },
            delete: {
              title: "Delete Error",
              message: "An error occurred while trying to delete auction.",
            },
            delete_all: {
              title: "Delete All Success",
              message: "An error occurred while trying to delete all auctions.",
            }
          },
        },
      }
    },
  },
}