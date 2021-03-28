Feature: Bot feature

  Scenario: Init bot with no registered users and handle /help message
    Given a bot
    When user sends "/help" to bot
    Then I recieve text equals to
    """
    "You can send me these commands:
/start
/feedback
/help
/history
/add

If you encounter any issues feel free to open an issue.
Or you can also send feedback via /feedback command."
    """
