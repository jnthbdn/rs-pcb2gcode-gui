use rusqlite::{Connection, Error, Row};

use crate::{
    dirs::get_config_path_to,
    tools::{basetool::BaseTool, drill::Drill, endmill::Endmill, vbit::VBit},
};

use gtk::glib;
static DB_FILE_NAME: &str = "tool_database.db";

#[derive(glib::Enum, Clone, Copy, Debug)]
#[enum_type(name = "DatabaseColumn")]
pub enum DatabaseColumn {
    ID,
    Name,
    Note,
    ShaftDiameter,
    ToolDiameter,
    SpindleSpeed,
    PassDepth,
    PlungeRate,
    FeedRate,

    ToolAngle,
    TipDiameter,
}

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
                    feed_rate REAL NOT NULL,
                    is_metric INTEGER NOT NULL
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
                    feed_rate REAL NOT NULL,
                    is_metric INTEGER NOT NULL
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
                    angle REAL NOT NULL,
                    is_metric INTEGER NOT NULL
                )",
            (),
        )?;

        Ok(Self { connection: conn })
    }

    pub fn add_endmill(&self, endmill: &Endmill) -> Result<(), rusqlite::Error> {
        self.connection.execute("INSERT OR ROLLBACK INTO endmill (name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", (
            &endmill.base_tool.name,
            &endmill.base_tool.note,
            endmill.base_tool.shaft_diameter,
            endmill.base_tool.tool_diameter,
            endmill.base_tool.spindle_speed,
            endmill.base_tool.pass_depth,
            endmill.base_tool.plunge_rate,
            endmill.base_tool.feed_rate,
            endmill.base_tool.unit.is_metric()
        ))?;

        Ok(())
    }

    pub fn add_drill(&self, drill: &Drill) -> Result<(), rusqlite::Error> {
        self.connection.execute("INSERT OR ROLLBACK INTO drill (name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", (
            &drill.base_tool.name,
            &drill.base_tool.note,
            drill.base_tool.shaft_diameter,
            drill.base_tool.tool_diameter,
            drill.base_tool.spindle_speed,
            drill.base_tool.pass_depth,
            drill.base_tool.plunge_rate,
            drill.base_tool.feed_rate,
            drill.base_tool.unit.is_metric()
        ))?;

        Ok(())
    }

    pub fn add_vbit(&self, vbit: &VBit) -> Result<(), rusqlite::Error> {
        self.connection.execute("INSERT OR ROLLBACK INTO vbit (name, note, shaft_diameter, tool_diameter, angle, tip_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)", (
            &vbit.base_tool.name,
            &vbit.base_tool.note,
            vbit.base_tool.shaft_diameter,
            vbit.base_tool.tool_diameter,
            vbit.tool_angle,
            vbit.tip_diameter,
            vbit.base_tool.spindle_speed,
            vbit.base_tool.pass_depth,
            vbit.base_tool.plunge_rate,
            vbit.base_tool.feed_rate,
            vbit.base_tool.unit.is_metric()
        ))?;

        Ok(())
    }

    pub fn get_all_endmills(&self) -> Result<Vec<Endmill>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric FROM endmill")?;

        let results = stmt.query_map([], Self::map_endmill)?;

        results.collect()
    }

    pub fn get_all_drills(&self) -> Result<Vec<Drill>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric FROM drill")?;

        let results = stmt.query_map([], Self::map_drill)?;

        results.collect()
    }

    pub fn get_all_vbits(&self) -> Result<Vec<VBit>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, angle, tip_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric FROM vbit")?;

        let results = stmt.query_map([], Self::map_vbit)?;

        results.collect()
    }

    pub fn get_endmill(&self, id: u32) -> Result<Option<Endmill>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric FROM endmill WHERE id=?1")?;

        let results = stmt.query_map([id], Self::map_endmill)?;

        let mut results: Vec<Endmill> =
            results.collect::<Result<Vec<Endmill>, rusqlite::Error>>()?;

        if results.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(results.pop().unwrap()))
        }
    }

    pub fn get_drill(&self, id: u32) -> Result<Option<Drill>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric FROM drill WHERE id=?1")?;

        let results = stmt.query_map([id], Self::map_drill)?;

        let mut results: Vec<Drill> = results.collect::<Result<Vec<Drill>, rusqlite::Error>>()?;

        if results.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(results.pop().unwrap()))
        }
    }

    pub fn get_vbit(&self, id: u32) -> Result<Option<VBit>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT id, name, note, shaft_diameter, tool_diameter, angle, tip_diameter, spindle_speed, pass_depth, plunge_rate, feed_rate, is_metric FROM vbit WHERE id=?1")?;

        let results = stmt.query_map([id], Self::map_vbit)?;

        let mut results: Vec<VBit> = results.collect::<Result<Vec<VBit>, rusqlite::Error>>()?;

        if results.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(results.pop().unwrap()))
        }
    }

    pub fn set_drill_column(
        &self,
        col: DatabaseColumn,
        value: String,
        id: u32,
    ) -> Result<(), Error> {
        self.connection
            .execute(
                format!(
                    "UPDATE drill SET {} = ? WHERE id = ?",
                    Self::database_column_to_str(col)
                )
                .as_str(),
                [value, id.to_string()],
            )
            .map(|_| ())
    }

    pub fn set_endmill_column(
        &self,
        col: DatabaseColumn,
        value: String,
        id: u32,
    ) -> Result<(), Error> {
        self.connection
            .execute(
                format!(
                    "UPDATE endmill SET {} = ? WHERE id = ?",
                    Self::database_column_to_str(col)
                )
                .as_str(),
                [value, id.to_string()],
            )
            .map(|_| ())
    }

    pub fn set_vbit_column(
        &self,
        col: DatabaseColumn,
        value: String,
        id: u32,
    ) -> Result<(), Error> {
        self.connection
            .execute(
                format!(
                    "UPDATE vbit SET {} = ? WHERE id = ?",
                    Self::database_column_to_str(col)
                )
                .as_str(),
                [value, id.to_string()],
            )
            .map(|_| ())
    }

    pub fn remove_drill(&self, id: u32) -> Result<(), Error> {
        self.connection
            .execute("DELETE FROM drill WHERE id=?1", [id])?;

        Ok(())
    }

    pub fn remove_endmill(&self, id: u32) -> Result<(), Error> {
        self.connection
            .execute("DELETE FROM endmill WHERE id=?1", [id])?;

        Ok(())
    }

    pub fn remove_vbit(&self, id: u32) -> Result<(), Error> {
        self.connection
            .execute("DELETE FROM vbit WHERE id=?1", [id])?;

        Ok(())
    }

    fn map_endmill(row: &Row) -> Result<Endmill, Error> {
        let is_metric: bool = row.get(9)?;

        if is_metric {
            Ok(Endmill::new_metric(
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
        } else {
            Ok(Endmill::new_imperial(
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
        }
    }

    fn map_drill(row: &Row) -> Result<Drill, Error> {
        let is_metric: bool = row.get(9)?;

        if is_metric {
            Ok(Drill::new_metric(
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
        } else {
            Ok(Drill::new_imperial(
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
        }
    }

    fn map_vbit(row: &Row) -> Result<VBit, Error> {
        let is_metric: bool = row.get(11)?;

        if is_metric {
            Ok(VBit::new_metric(
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
        } else {
            Ok(VBit::new_imperial(
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
        }
    }

    fn database_column_to_str(col: DatabaseColumn) -> String {
        match col {
            DatabaseColumn::ID => "id".to_string(),
            DatabaseColumn::Name => "name".to_string(),
            DatabaseColumn::Note => "note".to_string(),
            DatabaseColumn::ShaftDiameter => "shaft_diameter".to_string(),
            DatabaseColumn::ToolDiameter => "tool_diameter".to_string(),
            DatabaseColumn::SpindleSpeed => "spindle_speed".to_string(),
            DatabaseColumn::PassDepth => "pass_depth".to_string(),
            DatabaseColumn::PlungeRate => "plunge_rate".to_string(),
            DatabaseColumn::FeedRate => "feed_rate".to_string(),
            DatabaseColumn::ToolAngle => "angle".to_string(),
            DatabaseColumn::TipDiameter => "tip_diameter".to_string(),
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        match Self::new() {
            Ok(db) => db,
            Err(e) => {
                log::error!("Failed to create database. ({e})");
                std::process::exit(1);
            }
        }
    }
}
