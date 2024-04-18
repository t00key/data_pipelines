# Overview

Example ETL Data pipeline leveraging Rust to extract file from internet, transform data by adding geo-encoding & loading to a Postgres DB.


# Notes on geo-encoding:

* *Open street maps* - Free, but...Open street maps finds ~74% of locations in this demo pipeline.  Cannot find others that other services can
* *Google maps* - finds all addresses but requires subscription
* *mapbox* - free for first 100k/month but requires signup to obtain a API key
