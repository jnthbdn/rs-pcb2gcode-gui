use rusqlite::Connection;

use crate::{
    dirs::get_config_path_to,
    tools::{drill::Drill, endmill::Endmill, vbit::VBit},
};

static DB_FILE_NAME: &str = "tool_database.db";

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(get_config_path_to(DB_FILE_NAME).as_path())?;

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS endmill (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    note TEXT,
                    shaft_diameter REAL NOT NULL,
                    tool_diameter REAL NOT NULL,
                    spindle_speed REAL NOT NULL,
                    pass_depth REAL NOT NULL,
                    plunge_rate REAL NOT NULL,
                    feed_rate REAL NOT NULL
                )",
            (),
        )?;

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS drill (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    note TEXT,
                    shaft_diameter REAL NOT NULL,
                    tool_diameter REAL NOT NULL,
                    spindle_speed REAL NOT NULL,
                    pass_depth REAL NOT NULL,
                    plunge_rate REAL NOT NULL,
                    feed_rate REAL NOT NULL
                )",
            (),
        )?;

        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS vbit (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    note TEXT,
                    shaft_diameter REAL NOT NULL,
                    tool_diameter REAL NOT NULL,
                    spindle_speed REAL NOT NULL,
                    pass_depth REAL NOT NULL,
                    plunge_rate REAL NOT NULL,
                    feed_rate REAL NOT NULL,
                    tip_diameter REAL NOT NULL,
                    angle REAL NOT NULL
                )",
            (),
        )?;

        Ok(Self { connection: conn })
    }

    pub fn add_endmill(&self, endmill: &Endmill) -> Result<(), rusqlite::Error> {
        self.connection.execute("INSERT OR ROLLBACK INTO endmill (name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", (
            &endmill.base_tool.name,
            &endmill.base_tool.note,
            endmill.base_tool.shaft_diameter,
            endmill.base_tool.tool_diameter,
            endmill.base_tool.spindle_speed,
            endmill.base_tool.pass_depth,
            endmill.base_tool.plunge_rate,
            endmill.base_tool.feed_rate
        ))?;

        Ok(())
    }

    pub fn add_drill(&self, drill: &Drill) -> Result<(), rusqlite::Error> {
        self.connection.execute("INSERT OR ROLLBACK INTO drill (name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", (
            &drill.base_tool.name,
            &drill.base_tool.note,
            drill.base_tool.shaft_diameter,
            drill.base_tool.tool_diameter,
            drill.base_tool.spindle_speed,
            drill.base_tool.pass_depth,
            drill.base_tool.plunge_rate,
            drill.base_tool.feed_rate
        ))?;

        Ok(())
    }

    pub fn add_vbit(&self, vbit: &VBit) -> Result<(), rusqlite::Error> {
        self.connection.execute("INSERT OR ROLLBACK INTO vbit (name, note, shaft_diameter, tool_diameter, angle, tip_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)", (
            &vbit.base_tool.name,
            &vbit.base_tool.note,
            vbit.base_tool.shaft_diameter,
            vbit.base_tool.tool_diameter,
            vbit.tool_angle,
            vbit.tip_diameter,
            vbit.base_tool.spindle_speed,
            vbit.base_tool.pass_depth,
            vbit.base_tool.plunge_rate,
            vbit.base_tool.feed_rate
        ))?;

        Ok(())
    }

    pub fn get_all_endmills(&self) -> Result<Vec<Endmill>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate FROM endmill")?;

        let results = stmt.query_map([], |row| {
            Ok(Endmill::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ))
        })?;

        results.collect()
    }

    pub fn get_all_drills(&self) -> Result<Vec<Drill>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate FROM drill")?;

        let results = stmt.query_map([], |row| {
            Ok(Drill::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ))
        })?;

        results.collect()
    }

    pub fn get_all_vbits(&self) -> Result<Vec<VBit>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, angle, tip_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate FROM vbit")?;

        let results = stmt.query_map([], |row| {
            Ok(VBit::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
                row.get(10)?,
            ))
        })?;

        results.collect()
    }
}
