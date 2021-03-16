Feature: Animal feature

  Scenario: If we feed a hungry cat it will no longer be hungry
    Given A hungry cat
    When I feed the cat
    Then The cat is not hungry
