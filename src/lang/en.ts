export const en = {
  live_scraper: {
    item: {
      starting: "Starting Items",
      checking: "Checking: <blue>{{name}}</blue> <blue>{{count}}</blue>/<blue>{{total}}</blue>",
      deleting_orders: "Deleting Orders: <blue>{{count}}</blue>/<blue>{{total}}</blue>",
      sell: {
        deleting: "Deleting Sell Order: <blue>{{name}}</blue>",
        updating: "Updating Sell Order: <blue>{{name}}</blue> for <blue>{{price}}</plat></blue>",
        creating: "Creating Sell Order: <blue>{{name}}</blue> for <blue>{{price}}</plat></blue>",
      },
      buy: {
        deleting: "Deleting Buy Order: <blue>{{name}}</blue>",
        updating: "Updating Buy Order: <blue>{{name}}</blue> for <blue>{{price}}</plat></blue>",
        creating: "Creating Buy Order: <blue>{{name}}</blue> for <blue>{{price}}</plat></blue>",
      }
    },
    riven: {
      starting: "Starting Rivens",
      deleting: "Deleting Riven: <blue>{{name}}</blue>",
      searching: "Searching Riven: <blue>{{name}}</blue>",
      no_offers: "No offers found for: <blue>{{name}}</blue>",
      updating: "Updating Riven: <blue>{{name}}</blue> for <blue>{{price}}</plat></blue>",
      creating: "Creating Riven: <blue>{{name}}</blue> for <blue>{{price}}</plat></blue>",
    }
  },
  general: {
    months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
    total_quantity: "Total orders <italic>{{count}}</italic>",
    total_revenue: "Total revenue <italic>{{val, currency(USD)}}</italic>",
    total_revenue_average: "Average revenue <italic>{{count, number}}</italic>",
    this_year: "This year",
    last_year: "Last year",
    sales_label: "Sell",
    buy_label: "Buy",
    new_release_label: "Update {{ version }} is available",
    new_release_message: "Click here to install the new update",
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
      modes: {
        sales: {
          title: "Sales",
        },
        purchases: {
          title: "Purchases",
        },
        quantity: {
          title: "Quantity",
        },
        profit: {
          title: "Profit",
        },
      },
      context: {
        profit: "Profit: <blue>{{val, currency(USD)}}</blue>",
        profit_margin: "Profit Margin: <blue>{{val}} %</blue>",
        revenue_average: "Average Profit: <blue>{{val, currency(USD)}}</blue>",
        footer: "Sales <blue>{{sales}}</blue> | Purchases <blue>{{purchases}}</blue> | Trades <blue>{{trades}}</blue>",
      },
    },

    availableRivens: {
      weaponInfo: {
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
    labelTimeBage: {
      months: "{{months}} months ago",
      days: "{{days}} days ago",
      hours: "{{hours}} hours ago",
      minutes: "{{minutes}} minutes ago",
      seconds: "{{seconds}} seconds ago",
    },
    transactioncontrol: {
      title: "Transaction Control",
      price_scraper_start: "Start Price Scraper",
      price_scraper_last_run: "Price Scraper Was Last Run: <blue>{{date}}</blue>",
      price_scraper_running: "Running",
      live_trading_start: "Start Live Trading",
      live_trading_stop: "Stop Live Trading",
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
            fields: {
              order_mode: {
                label: "Order Mode",
                both_description: "Will buy and sell items",
                buy_description: "Will only buy items",
                sell_description: "Will only sell items",
                options: {
                  both: "Both",
                  buy: "Buy",
                  sell: "Sell",
                },
              },
              stock_mode: {
                label: "Stock Mode",
                all_description: "Will process all items",
                item_description: "Will only process items",
                riven_description: "Will only process rivens",
                options: {
                  all: "All",
                  item: "Item",
                  riven: "Riven",
                },
              },
            },
            title: "Live Trading",
            accordion_general: "General",
            accordion_whitelist: "Whitelist",
            accordion_blacklist: "Blacklist",
            volume_threshold: "Volume Threshold",
            volume_threshold_description: "Volume of items sold, set this to somewhere between 6-10, but default is 15",
            max_total_price_cap: "Max Total Price Cap",
            max_total_price_cap_description: "Total Plat it will put up WTB for",
            range_threshold: "Range Threshold",
            range_threshold_description: "Volume of plat profit per item flip the bot will look to buy/resell",
            riven_range_threshold: "Riven Range Threshold",
            riven_range_threshold_description: "Volume of profit for then riven to be sold.",
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
            report_to_wfm: "Report to WFM",
            report_to_wfm_description: "Will report buy/sell orders to WFM",
            ping_on_notif_description: "If you want to be pinged on discord.",
            webhook: "Webhook",
            webhook_description: "The webhook to send notifications to",
            enable: "Notify on a new conversation",
            enable_description: "If you want to be notified on a new conversation.",
            auto_trade: "Auto Trade",
            auto_trade_description: "Will try to add the items you buy/sell to the stock",
            filter: {
              tax: "Tax Range",
              mr: "MR Range",
            },
            save: "Save",
          },
          price_scraper: {},
          whisper_scraper: {
            title: "Whisper Scraper",
            accordion_general: "General",
            conversation: {
              system: {
                enable: {
                  title: "Enable System Notification",
                  description: "Will send a notification to the system on a new conversation",
                },
                title: {
                  title: "Title",
                  description: "The title of the notification",
                },
                content: {
                  title: "Content",
                  description: "Use <PLAYER_NAME> as the placeholder for the Warframe username ",
                },
              },
              discord: {
                enable: {
                  title: "Enable Discord Notification",
                  description: "Will send a notification to discord on a new conversation",
                },
                webhook: {
                  title: "Discord Webhook",
                  description: "The webhook to send notifications to",
                },
                user_ids: {
                  title: "User IDs",
                  description: "The user ids to ping separated by comma",
                },
                title: {
                  title: "Title",
                  description: "The title of the notification",
                },
                content: {
                  title: "Content",
                  description: "Use <PLAYER_NAME> as the placeholder for the Warframe username ",
                },
              }
            },
            save: "Save",
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
        export_logs: "Export Logs",
        status: {
          title: "Status",
          online: "Online",
          invisible: "Invisible",
          ingame: "Ingame",
        }
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
      chats: "Chats",
      warframe_market: "Warframe Market",
      auctions: "Contracts",
      debug: "Debug",
      buy_me_a_coffee: "Buy me a coffee",
    },
  },
  pages: {
    home: {
      stats_cards: {
        total: {
          title: "Total Profit",
          context: "Sales <blue>{{sales}}</blue> | Purchases <blue>{{purchases}}</blue> | <trade/> <blue>{{quantity}}</blue> | Margin <blue>{{profit_margin}}%</blue>",
        },
        total_chart: {
          title: "Total Profit",
        },
        today: {
          title: "Today Turnover",
          context: "Sales <blue>{{sales}}</blue> | Purchases <blue>{{purchases}}</blue> | <trade/> <blue>{{quantity}}</blue> | Margin <blue>{{profit_margin}}%</blue>",
        },
        today_chart: {
          title: "Today Profit",
        },
        last_days: {
          title: "Last {{days}} days",
        },
        best_selling: {
          title: "Best turnover product",
          context: "Name <blue>{{name}}</blue> | Sales <blue>{{sales}}</blue> | Purchases <blue>{{purchases}}</blue> | <trade/> <blue>{{quantity}}</blue> | Margin <blue>{{profit_margin}}%</blue>",
        },
        datagrid: {
          columns: {
            name: "Name",
            revenue: "Revenue",
            expense: "Expense",
            profit: "Profit",
            profit_margin: "Profit Margin",
          }
        }
      }
    },
    warframe_market: {
      tabs: {
        auctions: {
          title: "Contracts",
          tolltip: {
            refresh: "Refresh orders",
            delete_all: "Delete all orders",
          },
          notifaications: {
            import: {
              title: "Import",
              message: "Imported {{name}} auctions",
            },
            refresh: {
              title: "Refresh",
              message: "Auctions refreshed",
            },
            delete_all: {
              title: "Refresh",
              message: "Deleted {{count}} orders",
            }
          },
          prompt: {
            import: {
              title: "How much did you buy it for?",
              label: "Bought for",
              description: "Import auctions from warframe.market",
              placeholder: "Import",
            },
            delete_all: {
              title: "Delete All your orders",
              message: "Are you sure you want to delete all your orders? This cannot be undone.",
              confirm: "Delete",
              cancel: "Cancel",
            },
          },
          info: {
            inactive: "Is importede but is inactive",
            is_imported: "Is imported and is active",
            is_not_imported: "Is not imported",
          },
        },
        orders: {
          title: "Orders",
          tolltip: {
            refresh: "Refresh orders",
            delete_all: "Delete all orders",
            delete: "Delete order",
            buy_add_to_stock: "Buy and add to stock",
            sell_remove_from_stock: "Sell and remove from stock",
          },
          info: {
            buy: "Buy Orders: {{count}} ({{plat}}p)",
            sell: "Sell Orders: {{count}} ({{plat}}p)",
          },
          prompt: {
            delete_all: {
              title: "Delete All your orders",
              message: "Are you sure you want to delete all your orders? This cannot be undone.",
              confirm: "Delete",
              cancel: "Cancel",
            },
          },
          notifaications: {
            refresh: {
              title: "Refresh",
              message: "Orders refreshed",
            },
            delete_all: {
              title: "Refresh",
              message: "Deleted {{count}} orders",
            },
            createStockItem: {
              title: "Item added",
              message: "Item {{name}} added successfully",
            },
            sellStockItem: {
              title: "Item sold",
              message: "Item {{name}} sold successfully for {{price}}",
            },
            delete_ordre: {
              title: "Order deleted",
              message: "Order was deleted successfully",
            }
          },
          buttons: {
            refresh: "Refresh Orders",
          },
          sort: {
            buy: "Showing Buy Orders",
            sell: "Showing Sell Orders",
            all: "Showing All Orders",
          },
          rank_label: "Rank: <blue>{{rank}}</blue> of <blue>{{max_rank}}",
          plat_label: "<blue>{{plat}}</blue> <plat/>",
          quantity_label: "<qty/> <blue>{{quantity}}</blue>",
        },
      },
    },
    auth: {
      login: {
        title: "Warframe Market - Login",
        email: "Email",
        password: "Password",
        submit: "Login",
      },
    },
    live_trading: {
      tabs: {
        riven: {
          title: "Stock Rivens",
          total_listed_price: "Total Listed Price: <blue>{{price}}</blue>",
          total_purchase_price: "Total Purchase Price: <blue>{{price}}</blue>",
          total_profit: "Total Profit: <blue>{{price}}</blue>",
          info_boxs: {
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
                sell: {
                  title: "Sell",
                  prompt: {
                    title: "Sell Riven",
                    label: "Sold for",
                    description: "The price you sold the item for",
                  }
                },
                sell_for_listed_price: "Sell for listed price",
                is_private: {
                  enable: "Show on market",
                  disable: "Hide from market",
                },
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
          total_listed_price: "Total Listed Price: <blue>{{price}}</blue>",
          total_purchase_price: "Total Purchase Price: <blue>{{price}}</blue>",
          total_profit: "Total Profit: <blue>{{price}}</blue>",
          info_boxs: {
            pending_description: "Pending",
            live_description: "Live on market",
            to_low_profit_description: "Profit was too low to sell",
            no_offers_description: "No offers on market",
            inactive_description: "Is Private",
            no_buyers_description: "No buyers on market",
          },
          buttons: {
            resell: {
              label: "Resell",
              description_with_report: "Add item to your stock and market transaction",
              description_without_report: "Add item to your stock",
            }
          },
          fields: {
            quantity: {
              label: "Quantity",
              description: "Quantity of items"
            },
            price: {
              label: "Price",
              description: "Total price of the order"
            },
            rank: {
              label: "Rank",
              description: "Rank of the item"
            },
          },
          notifaications: {
            createStockItem: {
              title: "Item added",
              message: "Item {{name}} added successfully",
            },
            sellStockItem: {
              title: "Item sold",
              message: "Item {{name}} sold successfully for {{price}}",
            },
            deleteStockItem: {
              title: "Item deleted",
              message: "Item {{name}} deleted successfully",
            },
            updateStockItem: {
              title: "Item updated",
              message: "Item {{name}} updated successfully",
            },
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
                sell: {
                  title: "Sell",
                  prompt: {
                    title: "Sell Item",
                    label: "Sold for",
                    description: "The price you sold the item for",
                  }
                },
                is_hiding: {
                  enable: "Show on market",
                  disable: "Hide from market",
                },
                sell_for_listed_price: "Sell for listed price",
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
      },
    },
    wtbMessage: {
      copy_to_clipboard: "Copy to clipboard",
      wtb_message: "Generate Message",
      wtb_message_template: "WTB message template",
      wtb_message_max_length: "WTB message max length: {{length}}/{{maxLength}}",
      tooltip: {
        calculate_price: "Calculate price based on the lowest price on warframe.market",
        clear: "Clear list",
      },
      notifaications: {
        copied_to_clipboard: "Copied to clipboard",
      },
      modals: {
        generateWtbMessage: {
          title: "Generate WTB Message",
          description: "New rivens price",
          list_text: "{{name}}: <blue>{{previousPrice}}</blue> > <blue>{{price}}</blue> ",
          confirm: "Replace",
          cancel: "Cancel",
        }
      },
      prompt: {
        sell_price: {
          title: "Sell Price",
          label: "Sell Price",
          description: "The price you want to sell the riven for",
          placeholder: "Sell Price",
        },
        generateWtbMessage: {
          title: "Generate WTB Message",
          minSellers_label: "Min Sellers",
          minSellers_description: "Min sellers for the riven",
          minSellers_placeholder: "Min Sellers",
          lowestPrice_label: "Lowest Price",
          lowestPrice_description: "Lowest price for the riven",
          lowestPrice_placeholder: "Lowest Price",
          discount_label: "Discount",
          discount_description: "Discount for the riven",
          discount_placeholder: "Discount",
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
    },
    chats: {

      item: {
        un_read_messages: "<mail /> {{count}}",
        delete: "Leave Chat",
      },
      navbar: {
        back: "Back",
        options: "Options",
        delete: "Leave Chat",
        ignore: "Ignore User",
      },
      msgbox: {
        send: "Send",
        placeholder: "Type a message...",
        error: {
          msg_to_long: "Message is too long",
        },
      },
    },
  },
  success: {
    auth: {
      login_title: "Login success",
      login_message: "Welcome back {{name}}",
      logout_title: "Logout success",
      logout_message: "You have been logged out successfully",
    },
  },
  error: {
    auth: {
      login_title: "Login error",
      login_message: "Username or password is invalid {{name}}",
      logout_title: "Logout error",
      logout_message: "There was an error logging out. Please try again.",
    },
    rust: {
      title: "Error in {{component}}",
      message: "There was an error at {{loc}}. Please check the logs for more information.",
    },
  }
}
