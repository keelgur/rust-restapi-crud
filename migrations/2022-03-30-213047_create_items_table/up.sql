CREATE TABLE "materials" (
  "id" integer not null primary key autoincrement,
  "item" varchar not null,
  "density" float not null
);

INSERT INTO materials (item,density) VALUES
         ('basalt',20.5),
         ('cotton',10.5),
         ('opal',0.5),
         ('cobblestone',15.5),
         ('marble',20.0),
         ('copper',34.5),
         ('gold',50.5);
         ('stone',20.5),
         ('ruby',27.0),
         ('meteorite',15.5),
         ('antimatter',20.0),
         ('tanzanite',15.5),
         ('wood',20.0),
         ('wool',15.5),
         ('titanite',20.0),
