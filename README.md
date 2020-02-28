# diesel-geometry-postgis
Implement conversion from and to postgis for diesel

### Example 
Wrapping postgis for insert  
`let t = GeometryWrapper(postgis::ewkb::GeometryT::Point(postgis::ewkb::Point{ x: 0.0, y: 1.0, srid: None}));`  
  
_schema.rs_
```
table! {
    use diesel::sql_types::*;
    use diesel_geometry_postgis::sql_types::*;
    vector_datas (id) {
        id -> Int4,
        geom -> Geometry,
        properties -> Nullable<Jsonb>,
    }
}
```
