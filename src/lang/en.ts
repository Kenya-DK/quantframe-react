export const en = {
  months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
  notifications: {},
  common: {
    buttons: {
      save: {
        label: "Save",
      },
    },
  },
  enums: {
    stock_mode: {
      all: "All",
      riven: "Riven",
      item: "Item",
    },
    trade_mode: {
      buy: "Buy",
      sell: "Sell",
      wishlist: "Wishlist",
    },
    user_status: {
      online: "Online",
      ingame: "In game",
      invisible: "Offline",
    },
  },
  components: {
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
      },
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
          message: "An error occurred while trying to log out.",
        },
        update_settings: {
          title: "Update Settings Error",
          message: "An error occurred while trying to update settings.",
          wf_log_path_not_exist: "Warframe log path does not exist",
        },
      },
      success: {
        logout: {
          title: "Logout Success",
          message: "You have successfully logged out.",
        },
        update_settings: {
          title: "Update Settings Success",
          message: "Settings have been successfully updated.",
        },
      },
    },
    clock: {
      gmt: "GMT: <blue>{{time}}</blue>",
      time_until_midnight: "Time until midnight (GMT): <blue>{{time}}</blue>",
    },
    layout: {
      log_in: {
        navbar: {
          home: "Home",
          debug: "Debug",
        },
      },
    },
    forms: {
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
            error: "Password must include at least 6 characters",
          },
        },
        buttons: {
          submit: "Log In",
        },
      },
      settings: {
        tabs: {
          live_trading: {
            title: "Live Trading",
            general: {
              title: "General",
              fields: {
                auto_trade: {
                  label: "Auto Trade (WIP)",
                  error: "Invalid auto trade",
                  tooltip: "Automatically add/sell stock if true",
                },
                auto_delete: {
                  label: "Auto Delete",
                  error: "Invalid auto delete",
                  tooltip: "Automatically delete stock items",
                },
                should_delete_other_types: {
                  label: "Should Delete Other Trade Types",
                  error: "Invalid should delete other trade types",
                  tooltip:
                    "Will delete other trade types if true example: if buy is enabled will delete sell/wishlist items if they are not blacklisted",
                },
                report_to_wfm: {
                  label: "Report to Warframe Market",
                  error: "Invalid report to Warframe Market",
                  tooltip: "Will add a transaction to Warframe Market",
                },
                trade_modes: {
                  label: "Trade Mode",
                  description: "How the bot will trade",
                  error: "Invalid trade mode",
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
              },
            },
            item: {
              title: "Stock Item",
              wtb: {
                title: "WTB",
                fields: {
                  volume_threshold: {
                    label: "Volume Threshold",
                    placeholder: "Volume Threshold",
                    error: "Invalid volume threshold",
                    tooltip: "Minimum volume to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#volume-threshold",
                  },
                  profit_threshold: {
                    label: "Profit Threshold",
                    placeholder: "Profit Threshold",
                    error: "Invalid profit threshold",
                    tooltip: "Minimum profit to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#profit-threshold",
                  },
                  avg_price_cap: {
                    label: "Average Price Cap",
                    placeholder: "Average Price Cap",
                    error: "Invalid average price cap",
                    tooltip: "Maximum average price to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#average-price-cap",
                  },
                  trading_tax_cap: {
                    label: "Trading Tax Cap",
                    placeholder: "Trading Tax Cap",
                    error: "Invalid trading tax cap",
                    tooltip: "Maximum tax to consider for trading",
                    link: "https://example.com/trading-tax-cap",
                  },
                  max_total_price_cap: {
                    label: "Max Total Price Cap",
                    placeholder: "Max Total Price Cap",
                    error: "Invalid max total price cap",
                    tooltip: "Maximum total price to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#max-total-price-cap",
                  },
                  price_shift_threshold: {
                    label: "Price Shift Threshold",
                    placeholder: "Price Shift Threshold",
                    error: "Invalid price shift threshold",
                    tooltip: "Minimum price shift to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#price-shift-threshold",
                  },
                  buy_quantity: {
                    label: "Quantity of goods",
                    placeholder: "Quantity of goods",
                    error: "Invalid quantity of goods",
                    tooltip: "The quantity of goods to buy",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#quantity-of-goods",
                  },
                  min_wtb_profit_margin: {
                    label: "Profit Margin Threshold",
                    placeholder: "Profit Margin Threshold",
                    error: "Invalid profit margin threshold",
                    tooltip: "Minimum profit margin for WTB trades",
                    link: "https://quantframe.app/features/live-trading/settings/item/wtb#profit-margin-threshold",
                  },
                  wts: {
                    title: "WTS",
                    fields: {},
                  },
                },
              },
              wts: {
                title: "WTS",
                fields: {
                  min_sma: {
                    label: "Minimum SMA",
                    placeholder: "Minimum SMA",
                    error: "Invalid minimum SMA",
                    tooltip: "Minimum Simple Moving Average to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wts#min-sma",
                  },
                  min_profit: {
                    label: "Minimum Profit",
                    placeholder: "Minimum Profit",
                    error: "Invalid minimum profit",
                    tooltip: "Minimum profit to consider for trading",
                    link: "https://quantframe.app/features/live-trading/settings/item/wts#min-profit",
                  },
                },
              },
            },
            riven: {
              title: "Stock Riven",
              fields: {
                min_profit: {
                  label: "Minimum Profit",
                  placeholder: "Minimum Profit",
                  error: "Invalid minimum profit",
                  tooltip: "Minimum profit to consider for trading",
                  link: "https://quantframe.app/features/live-trading/settings/riven/wts#min-profit",
                },
                threshold_percentage: {
                  label: "Threshold Percentage",
                  placeholder: "Threshold Percentage",
                  error: "Invalid threshold percentage",
                  tooltip: "Percentage threshold for trading",
                  link: "https://quantframe.app/features/live-trading/settings/riven/wts#minimum-price-shift",
                },
                limit_to: {
                  label: "Limit To",
                  placeholder: "Limit To",
                  error: "Invalid limit to",
                  tooltip: "Limit the number of trades",
                  link: "https://quantframe.app/features/live-trading/settings/riven/wts#limit-to",
                },
                update_interval: {
                  label: "Update Interval (seconds)",
                  placeholder: "Update Interval",
                  error: "Invalid update interval",
                  tooltip: "Interval for updating riven trades",
                  link: "https://quantframe.app/features/live-trading/settings/riven/wts#update-interval",
                },
              },
            },
          },
        },
      },
    },
    modals: {
      base: {
        buttons: {
          confirm: "Confirm",
          cancel: "Cancel",
        },
      },
    },
  },
  context: {},
  pages: {
    debug: {
      tabs: {
        logging: {
          title: "Logging",
          datatable: {
            columns: {
              command: "Command",
              count: "Count",
              actions: {
                title: "Actions",
                buttons: {
                  remove_tooltip: "Remove Log on command",
                },
              },
            },
          },
          buttons: {
            add_log: "Add Log",
          },
          prompt: {
            name: {
              title: "Add Log",
              fields: {
                name: {
                  label: "Log Name",
                },
              },
            },
          },
        },
        states: {
          title: "States",
          accordions: {
            app_info: "App Info",
            app_error: "App Error",
            settings: "Settings",
            alerts: "Alerts",
            user: "User",
          },
        },
      },
    },
    auth: {
      errors: {
        login: {
          title: "Login Error",
          TooManyRequests: "Too many requests, please try again later",
          InvalidCredentials: "Password invalid",
          banned: "You are banned",
          ban_reason: "<red>Reason: {{reason}}</red>",
          verification: "You need to verify your account Do this on Warframe Market website",
        },
      },
      success: {
        login: {
          title: "Login Success",
          message: "Welcome back! {{name}}",
        },
      },
    },
  },
};
