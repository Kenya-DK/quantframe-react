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
    },
    transactionRevenueChart: {
      revenue_label: "Revenue: {{val, number}}",
      quantity_label: "Quantity: {{count}}",
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
      rank_description: "Rank of the mod",
      report: "Report",
      report_description: "w/o Reporting",
      total_listed_price: "Total Listed Price: <italic>{{price}}</italic>",
      total_purchase_price: "Total Purchase Price: <italic>{{price}}</italic>",
      buttons: {
        buy: "Buy",
        sell: "Sell",
      },
      datagrid: {
        columns: {
          name: "Name",
          price: "Price Per Unit",
          listed_price: "Listed Price",
          owned: "Owned",
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
            ping_on_notif: "Ping on Notification",
            ping_on_notif_description: "If you want to be pinged on discord.",
            webhook: "Webhook",
            webhook_description: "The webhook to send notifications to",
            save: "Save",
          },
          price_scraper: {},
          wisper: {},
        },
      },
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
        settings: "Settings",
        logout: "Log Out",
      }
    },
    navigation: {
      home: "Home",
      live_trading: "Live Trading",
      statistics: "Statistics",
      warframe_market: "Warframe Market",
      rivens: "Rivens",
      debug: "Debug",
    },
  },
  pages: {
    home: {
      stats_cards: {
        total_revenue_title: "Total Turnover",
        total_sales: "Total Sales: <italic>{{val}}</italic>",

        last_days_title: "Last {{days}} days",
        today_revenue_title: "Today revenue",
        open_orders_title: "Open orders",
        best_selling_product_title: "Best selling product",
        average_order_revenue: "Average order revenue <italic>{{val, currency(USD)}}</italic>",
        average_orders_per_month: "Average orders per month <italic>{{val, currency(USD)}}</italic>",
        revenue_compare_to_last_year_less: "Revenue <italic>{{val, currency(USD)}}</italic> less than last year",
        revenue_compare_to_last_year_more: "Revenue <italic>{{val, currency(USD)}}</italic> more than last year",
        completed_orders_today: "Completed orders <italic>{{count}}</italic>",
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
  error: {}
}
