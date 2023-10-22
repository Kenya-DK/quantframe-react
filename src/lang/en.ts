export const en = {
  general: {
    months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
    total_quantity: "Total orders <italic>{{count}}</italic>",
    total_revenue: "Total revenue <italic>{{val, currency(USD)}}</italic>",
    total_revenue_average: "Average revenue <italic>{{count, number}}</italic>",
    this_year: "This year",
    last_year: "Last year",
    sales_label: "Sell",
    buy_label: "Buy",
  },
  components: {
    forms: {
      riven: {
        weapon_name: "Weapon Name",
        attributes: "Attributes",
        mod_name: "Mod Name",
        mod_rank: "Mod Rank",
        mastery_rank: "Mastery Rank",
        re_rolls: "Re-Rolls",
        polarity: "Polarity",
        bought: "Bought",
        add_attribute: "Add Attribute",
        save: "Save",
      }
    },
    transactionRevenueChart: {
      revenue_label: "Revenue: {{val, number}}",
      quantity_label: "Quantity: {{count}}",
    },
    availableRivens: {
      weaponInfo: {
        weekly_rivens: {
          range: "Weekly Rivens Range <blue>{{start}}</blue> - <blue>{{end}}</blue>",
        },
        wfm: {
          sellers: "Sellers: <blue>{{sellers}}</blue>",
          username: "Username: <blue>{{username}}</blue>",
          lowestPrice: "Lowest Price: <blue>{{price}}</blue>",
        },
        title: "Weapon Info",
        weapon_name: "Weapon Name",
        mastery_rank: "Mastery Rank",
        riven_type: "Riven Type",
        group: "Group",
      },
      datagrid: {
        columns: {
          name: "Name",
          riven_type: "Riven Type",
          mastery_level: "Max Mastery Level",
          group: "Group",
          actions: {
            title: "Actions",
            add: "Add",
          },
        }
      }
    },
    searchfield: {
      title: "Search",
      placeholder: "Search...",
      buttons: {
        search: "Search",
        filter: "Filter",
        create: "Create",
      }
    },
    searchItemField: {
      title: "Search item",
      description: "Search for an item",
      placeholder: "Search item",
      no_results: "No results",
    },
    multiSelectListBox: {
      add_all: "Add all",
      remove_all: "Remove all",
      name: "Name",
    },
    inventory: {
      price: "Price",
      price_description: "Price per unit",
      quantity: "Quantity",
      quantity_description: "Quantity of items",
      rank: "Mod Rank",
      report_tooltip: "Will try to add the transaction to the market",
      report: "Report",
      report_description: "w/o Reporting",
      total_listed_price: "Total Listed Price: <italic>{{price}}</italic>",
      total_purchase_price: "Total Purchase Price: <italic>{{price}}</italic>",
      buttons: {
        buy: "Buy",
        buy_tooltip: "Will only add a buy transaction",
        sell: "Sell",
        sell_tooltip: "Will only add a sell transaction",
        resell: "Resell",
        resell_tooltip: "This will try to resell the item on the market",
      },
      datagrid: {
        columns: {
          name: "Name",
          price: "Price Per Unit",
          listed_price: "Listed Price",
          owned: "Owned",
          minium_price: {
            title: "Min Price",
            description: "Minium price to sell the item for",
            prompt: {
              title: "Minium price",
              minium_price_label: "Minium price",
            }
          },
          actions: {
            title: "Actions",
            sell: "Sell",
            sell_report: "Sell & Report",
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
    transactioncontrol: {
      title: "Transaction Control",
      price_scraper_start: "Start Price Scraper",
      price_scraper_running: "Price Scraper Running",
      live_trading_start: "Start Live Trading",
      live_trading_stop: "Stop Live Trading",
      wisper_start: "Start Whisper",
      wisper_stop: "Stop Whisper",
    },
    modals: {
      prompt: {
        confirmLabel: "Confirm",
        cancelLabel: "Cancel",
      },
      settings: {
        panels: {
          general: {
            title: "General",
          },
          live_trading: {
            title: "Live Trading",
            accordion_general: "General",
            accordion_whitelist: "Whitelist",
            accordion_blacklist: "Blacklist",
            volume_threshold: "Volume Threshold",
            volume_threshold_description: "Volume of items sold, set this to somewhere between 6-10",
            max_total_price_cap: "Max Total Price Cap",
            max_total_price_cap_description: "Total Plat it will put up WTB for",
            range_threshold: "Range Threshold",
            range_threshold_description: "Volume of plat profit per item flip the bot will look to buy/resell",
            riven_range_threshold: "Riven Range Threshold",
            riven_range_threshold_description: "Volume of plat profit per item flip the bot will look to buy/resell",
            avg_price_cap: "Average Price Cap",
            avg_price_cap_description: "Average price of the items it wants to buy",
            price_shift_threshold: "Price Shift Threshold",
            price_shift_threshold_description: "Always have this at -1",
            whitelist_label: "Whitelist",
            whitelist_description: "Need Info",
            whitelist_placeholder: "None",
            blacklist_label: "Blacklist",
            blacklist_description: "Need Info",
            blacklist_placeholder: "None",
            strict_whitelist: "Strict Whitelist",
            strict_whitelist_description: "Need Info",
            ping_on_notif_description: "If you want to be pinged on discord.",
            webhook: "Webhook",
            webhook_description: "The webhook to send notifications to",
            filter: {
              tax: "Tax Range",
              mr: "MR Range",
            },
            save: "Save",
          },
          price_scraper: {},
          whisper_scraper: {
            title: "Whisper Scraper",
          },
        },
      },
    },
    auction: {
      import_tooltip: "Import auctions from warframe.market",
      mastery_rank: "MR: <blue>{{mastery_rank}}</blue>",
      rank: "Ranks: <blue>{{rank}}</blue>",
      re_rolls: "Re-rolls: <blue>{{re_rolls}}</blue>",
      polarity: "Polarity: <blue>{{polarity}}</blue>",
      bought: "Bought for: <blue>{{bought}}</blue>",
      selling_price: "Selling price: <blue>{{price}}</blue>",
      buyout_price: "Buyout Price: <blue>{{price}}</blue>",
      starting_price: "Starting Price: <blue>{{price}}</blue>",
      top_bid: "Top bid: <blue>{{price}}</blue>",
    }
  },
  context: {
    wisper: {
      title: "Wisper",
      message: "Wisper {{name}}",
      error_title: "Wisper Error",
      error_message: "There was an error with the wisper. Please check the logs for more information.",
    },
    live_scraper: {
      error_title: "Live Scraper Error",
      error_message: "There was an error with the live scraper. Please check the logs for more information.",
    },
    price_scraper: {
      error_title: "Price Scraper Error",
      error_message: "There was an error with the price scraper. Please check the logs for more information.",
    },
    tauri: {
      notifications: {
        session_expired: "Session Expired",
        session_expired_message: "Your session has expired, please login again",
      },
    },
  },
  layout: {
    header: {
      title: "QuantFrame",
      profile: {
        settings: "Settings",
        logout: "Log Out",
        open_logs_folder: "Open Logs Folder",
      },
      notifications: {
        settings_updated: "Settings Updated",
        settings_updated_message: "Settings updated successfully",
      },
    },
    navigation: {
      home: "Home",
      live_trading: "Live Trading",
      statistics: "Statistics",
      warframe_market: "Warframe Market",
      auctions: "Contracts",
      debug: "Debug",
    },
  },
  pages: {
    home: {
      stats_cards: {
        total: {
          title: "Total Turnover",
          context: "Sales <italic>{{sales}}</italic> | Buy <italic>{{buy}}</italic> | <qty/> <italic>{{quantity}}</italic>",
        },
        today: {
          title: "Today Turnover",
          context: "Sales <italic>{{sales}}</italic> | Buy <italic>{{buy}}</italic> | <qty/> <italic>{{quantity}}</italic>",
        },
        best_selling: {
          title: "Best turnover product",
          context: "Name <italic>{{name}}</italic> | Sales <italic>{{sales}}</italic> | Buy <italic>{{buy}}</italic> | <qty/> <italic>{{quantity}}</italic>",
        },
        total_revenue_title: "Total Turnover",
        total_sales_old: "Total: Sales: <italic>{{val}}</italic> Invoices: <italic>{{val}}</italic>",
        total_sales: "Sales <italic>{{sales}}</italic> | Buy <italic>{{buy}}</italic> | Quantity <italic>{{quantity}}</italic>",

        today_revenue_title: "Today Turnover",
        today_revenue_context: "Today Turnover",

        last_days_title: "Last {{days}} days",
        open_orders_title: "Open orders",
        best_selling_product_title: "Best selling product",
        no_data: "No data",
        average_order_revenue: "Average order revenue <italic>{{val, currency(USD)}}</italic>",
        average_orders_per_month: "Average orders per month <italic>{{val, currency(USD)}}</italic>",
        revenue_compare_to_last_year_less: "Revenue <italic>{{val, currency(USD)}}</italic> less than last year",
        revenue_compare_to_last_year_more: "Revenue <italic>{{val, currency(USD)}}</italic> more than last year",
        completed_orders_today: "Completed orders <italic>{{count}}</italic>",
      }
    },
    warframe_market: {
      tabs: {
        auctions: {
          title: "Contracts",
          notifaications: {
            import_title: "Import",
            import_message: "Imported {{name}} auctions",
            refresh_title: "Refresh",
            refresh_message: "Contracts refreshed",
          },
          buttons: {
            refresh: "Refresh Contracts",
          },
          prompt: {
            import: {
              title: "Import",
              label: "Import",
              description: "Import auctions from warframe.market",
              placeholder: "Import",
            },
          },
        },
        orders: {
          title: "Orders",
          notifaications: {
            refresh_title: "Refresh",
            refresh_message: "Orders refreshed",
          },
          buttons: {
            refresh: "Refresh Orders",
          },
        },
      },
      rank_label: "Rank: {{rank}} of {{max_rank}}",
      plat_label: "Plat: <plat_html>{{plat}}</plat_html>",
      buy_label: "WTB",
      sell_label: "WTS",
      buttons: {
        delete: "Delete",
        edit: "Edit",
        bought: "Bought",
        visible: "Visible",
        sold: "Sold",
        hidden: "Hidden",
      }
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
    live_trading: {
      tabs: {
        riven: {
          title: "Stock Rivens",
          infos: {
            pending_description: "Pending",
            live_description: "Live on market",
            to_low_profit_description: "Profit was too low to sell",
            no_offers_description: "No offers on market",
            inactive_description: "Is Private",
          },
          notifaications: {
            copy_wts: {
              title: "Copy WTS",
              message: "{{msg}} copied to clipboard",
            },
            sell_title: "Sell",
            sell_message: "Riven {{name}} sold successfully for {{price}}",
            update_title: "Update",
            update_message: "Riven {{name}} updated successfully",
            delete_title: "Delete",
            delete_message: "Riven {{name}} deleted successfully",
            create_title: "Create",
            create_message: "Riven {{name}} created successfully",
          },
          buttons: {
            create: "Add Riven",
            create_wtb_message: "Create WTB Message",
          },
          datagrid: {
            context_menu: {
              copy_wts: "Create WTS Message",
            },
            columns: {
              name: "Name",
              mastery_rank: "MR",
              rank: "Rank",
              re_rolls: {
                title: "Re-Rolls",
                match: "Match Rivens with min {{min}} and max {{max}} re-rolls",
                any: "Match Rivens with any re-rolls",
                prompt: {
                  title: "Match Rivens with re-rolls",
                  enabled_label: "Should the re-rolls be used in the search",
                  enabled_description: "Should the re-rolls be used in the search",
                  min_label: "Minimun",
                  min_description: "Minimun Re-Rolls",
                  min_placeholder: "0",
                  max_label: "Maximun",
                  max_description: "Maximun Re-Rolls",
                  max_placeholder: "0",
                }
              },
              minium_price: {
                title: "Min Price",
                description: "Minium price to sell the riven for",
                prompt: {
                  title: "Minium price",
                  minium_price_label: "Minium price",
                }
              },
              price: "Price",
              listed_price: "Listed Price",
              attributes: "Attributes",
              actions: {
                title: "Actions",
                sell: "Sell",
                delete: {
                  title: "Delete",
                  message: "Are you sure you want to delete this riven?",
                  buttons: {
                    confirm: "Delete",
                    cancel: "Cancel",
                  }
                }
              },
            }
          }
        },
        item: {
          title: "Stock Items",

        },
      },
    },
    wtbMessage: {
      copy_to_clipboard: "Copy to clipboard",
      prompt: {
        sell_price: {
          title: "Sell Price",
          label: "Sell Price",
          description: "The price you want to sell the riven for",
          placeholder: "Sell Price",
        },
      },
      datagrid: {
        columns: {
          name: "Name",
          bought_price: "Bought Price",
          riven_type: "Riven Type",
          mastery_level: "Max Mastery Level",
          group: "Group",
          actions: {
            title: "Actions",
            edit: "Edit",
            delete: "Delete",
          },
        }
      }
    }
  },
  success: {
    auth: {
      login_title: "Login success",
      login_message: "Welcome back {{name}}",
      logout_title: "Logout success",
      logout_message: "You have been logged out successfully",
    },
    invantory: {
      create_title: "Item added",
      create_message: "Item {{name}} added successfully",
      update_title: "Item updated",
      update_message: "Item {{name}} updated successfully",
      delete_title: "Item deleted",
      delete_message: "Item {{name}} deleted successfully",
      sell_title: "Item sold",
      sell_message: "Item {{name}} sold successfully for {{price}}",
    }
  },
  error: {
    auth: {
      login_title: "Login error",
      login_message: "Username or password is invalid {{name}}",
      logout_title: "Logout error",
      logout_message: "There was an error logging out. Please try again.",
    }
  }
}
