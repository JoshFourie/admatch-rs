# admatch-rs
Library for matching advertisements to users.

There are three core modules: User, Advertiser and Matcher.

User and Advertiser are responsible for generating the schema from raw data, which the Matcher then operates upon.

User is composed of Raw and Profile, which disambiguates into Interests and Attributes. The header only declares get_profile() for the module. Raw constructs a Profile from JSON or anything else that cooperates with the serde::Deserializer.

Modules are split because I expect that we will improve the schema development dramatically over time. A further module for a schema might be useful. 

The next task is to construct the minimum for the advertisement module, and then to develop the matching algorithm which might require more coercion for the User and Advertiser schema.