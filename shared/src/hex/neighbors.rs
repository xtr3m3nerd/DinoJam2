use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::hex_grid::{
    axial::AxialPos,
    neighbors::{HexDirection, HEX_DIRECTIONS},
    offset::{ColEvenPos, ColOddPos, RowEvenPos, RowOddPos},
};
use bevy_ecs_tilemap::prelude::*;

// pulled from
// https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/src/helpers/hex_grid/neighbors.rs
// Too lazy to try to work out how to just pull the specific commit without breaking version
// compatibility.
// Should work for our needs here

/// Stores some data `T` associated with each neighboring hex cell, if present.
#[derive(Debug, Default)]
pub struct HexNeighbors<T> {
    zero: Option<T>,
    one: Option<T>,
    two: Option<T>,
    three: Option<T>,
    four: Option<T>,
    five: Option<T>,
}

impl<T> HexNeighbors<T> {
    /// Get an item that lies in a particular hex direction, specified by a [`HexDirection`].
    ///
    /// Will be `None` if no such items exists.
    pub fn get(&self, direction: HexDirection) -> Option<&T> {
        use HexDirection::*;
        match direction {
            Zero => self.zero.as_ref(),
            One => self.one.as_ref(),
            Two => self.two.as_ref(),
            Three => self.three.as_ref(),
            Four => self.four.as_ref(),
            Five => self.five.as_ref(),
        }
    }

    /// Get a mutable reference to an item that lies in a particular hex direction.
    ///
    /// Will be `None` if no such items exists.
    pub fn get_inner_mut(&mut self, direction: HexDirection) -> Option<&mut T> {
        use HexDirection::*;
        match direction {
            Zero => self.zero.as_mut(),
            One => self.one.as_mut(),
            Two => self.two.as_mut(),
            Three => self.three.as_mut(),
            Four => self.four.as_mut(),
            Five => self.five.as_mut(),
        }
    }

    /// Get a mutable reference to the optional item that lies in a particular hex direction.
    ///
    /// Will be `None` if no such items exists.
    pub fn get_mut(&mut self, direction: HexDirection) -> &mut Option<T> {
        use HexDirection::*;
        match direction {
            Zero => &mut self.zero,
            One => &mut self.one,
            Two => &mut self.two,
            Three => &mut self.three,
            Four => &mut self.four,
            Five => &mut self.five,
        }
    }

    /// Set item that lies in a particular hex direction.
    ///
    /// This does an [`Option::replace`](Option::replace) under the hood.
    pub fn set(&mut self, direction: HexDirection, data: T) {
        self.get_mut(direction).replace(data);
    }

    /// Iterate over neighbors, in the order specified by [`HEX_DIRECTIONS`].
    ///
    /// If a neighbor is `None`, this iterator will skip it.
    pub fn iter(&self) -> impl Iterator<Item = &'_ T> + '_ {
        HEX_DIRECTIONS
            .into_iter()
            .filter_map(|direction| self.get(direction))
    }

    /// Applies the supplied closure `f` with an [`and_then`](std::option::Option::and_then) to each
    /// neighbor element, where `f` takes `T` by value.
    pub fn and_then<U, F>(self, f: F) -> HexNeighbors<U>
    where
        F: Fn(T) -> Option<U>,
    {
        HexNeighbors {
            zero: self.zero.and_then(&f),
            one: self.one.and_then(&f),
            two: self.two.and_then(&f),
            three: self.three.and_then(&f),
            four: self.four.and_then(&f),
            five: self.five.and_then(&f),
        }
    }

    /// Applies the supplied closure `f` with an [`and_then`](std::option::Option::and_then) to each
    /// neighbor element, where `f` takes `T` by reference.
    pub fn and_then_ref<'a, U, F>(&'a self, f: F) -> HexNeighbors<U>
    where
        F: Fn(&'a T) -> Option<U>,
    {
        HexNeighbors {
            zero: self.zero.as_ref().and_then(&f),
            one: self.one.as_ref().and_then(&f),
            two: self.two.as_ref().and_then(&f),
            three: self.three.as_ref().and_then(&f),
            four: self.four.as_ref().and_then(&f),
            five: self.five.as_ref().and_then(&f),
        }
    }

    /// Applies the supplied closure `f` with a [`map`](std::option::Option::map) to each
    /// neighbor element, where `f` takes `T` by reference.
    pub fn map_ref<'a, U, F>(&'a self, f: F) -> HexNeighbors<U>
    where
        F: Fn(&'a T) -> U,
    {
        HexNeighbors {
            zero: self.zero.as_ref().map(&f),
            one: self.one.as_ref().map(&f),
            two: self.two.as_ref().map(&f),
            three: self.three.as_ref().map(&f),
            four: self.four.as_ref().map(&f),
            five: self.five.as_ref().map(&f),
        }
    }

    /// Generates `HexNeighbors<T>` from a closure that takes a hex direction and outputs
    /// `Option<T>`.
    pub fn from_directional_closure<F>(f: F) -> HexNeighbors<T>
    where
        F: Fn(HexDirection) -> Option<T>,
    {
        use HexDirection::*;
        HexNeighbors {
            zero: f(Zero),
            one: f(One),
            two: f(Two),
            three: f(Three),
            four: f(Four),
            five: f(Five),
        }
    }
}

impl HexNeighbors<TilePos> {
    /// Returns neighboring tile positions, given a coordinate system.
    ///
    /// In general, if you know which coordinate system you are using, it will be more efficient to
    /// use one of:
    ///     * [`HexNeighbors::get_neighboring_positions_standard`]
    ///     * [`HexNeighbors::get_neighboring_positions_row_even`]
    ///     * [`HexNeighbors::get_neighboring_positions_row_odd`]
    ///     * [`HexNeighbors::get_neighboring_positions_col_even`]
    ///     * [`HexNeighbors::get_neighboring_positions_col_odd`]
    ///
    /// A tile position will be `None` for a particular direction, if that neighbor would not lie
    /// on the map.
    pub fn get_neighboring_positions(
        tile_pos: &TilePos,
        map_size: &TilemapSize,
        hex_coord_sys: &HexCoordSystem,
    ) -> HexNeighbors<TilePos> {
        match hex_coord_sys {
            HexCoordSystem::RowEven => {
                HexNeighbors::get_neighboring_positions_row_even(tile_pos, map_size)
            }
            HexCoordSystem::RowOdd => {
                HexNeighbors::get_neighboring_positions_row_odd(tile_pos, map_size)
            }
            HexCoordSystem::ColumnEven => {
                HexNeighbors::get_neighboring_positions_col_even(tile_pos, map_size)
            }
            HexCoordSystem::ColumnOdd => {
                HexNeighbors::get_neighboring_positions_col_odd(tile_pos, map_size)
            }
            HexCoordSystem::Row | HexCoordSystem::Column => {
                HexNeighbors::get_neighboring_positions_standard(tile_pos, map_size)
            }
        }
    }

    /// Returns neighboring tile positions. This works for maps using [`HexCoordSystem::Row`] and
    /// [`HexCoordSystem::Column`].
    ///
    /// For maps using [`HexCoordSystem::RowEven`], [`HexCoordSystem::ColEven`],
    /// [`HexCoordSystem::RowOdd`], [`HexCoordSystem::RowOdd`], use one of:
    ///     * [`HexNeighbors::get_neighboring_positions_row_even`]
    ///     * [`HexNeighbors::get_neighboring_positions_row_odd`]
    ///     * [`HexNeighbors::get_neighboring_positions_col_even`]
    ///     * [`HexNeighbors::get_neighboring_positions_col_odd`]
    /// (Or, just don't use a map that with a odd/even coordinate system, and prefer to use one
    /// with  [`HexCoordSystem::Row`] or [`HexCoordSystem::Column`]).
    ///
    /// A tile position will be `None` for a particular direction, if that neighbor would not lie
    /// on the map.
    pub fn get_neighboring_positions_standard(
        tile_pos: &TilePos,
        map_size: &TilemapSize,
    ) -> HexNeighbors<TilePos> {
        let axial_pos = AxialPos::from(tile_pos);
        let f = |direction| {
            axial_pos
                .offset(direction)
                .as_tile_pos_given_map_size(map_size)
        };
        HexNeighbors::from_directional_closure(f)
    }

    /// Returns neighboring tile positions on a map using [`HexCoordSystem::RowEven`].
    ///
    /// A tile position will be `None` for a particular direction, if that neighbor would not lie
    /// on the map.
    pub fn get_neighboring_positions_row_even(
        tile_pos: &TilePos,
        map_size: &TilemapSize,
    ) -> HexNeighbors<TilePos> {
        let axial_pos = AxialPos::from(RowEvenPos::from(tile_pos));
        let f = |direction| {
            RowEvenPos::from(axial_pos.offset(direction)).as_tile_pos_given_map_size(map_size)
        };
        HexNeighbors::from_directional_closure(f)
    }

    /// Returns neighboring tile positions on a map using [`HexCoordSystem::RowOdd`].
    ///
    /// A tile position will be `None` for a particular direction, if that neighbor would not lie
    /// on the map.
    pub fn get_neighboring_positions_row_odd(
        tile_pos: &TilePos,
        map_size: &TilemapSize,
    ) -> HexNeighbors<TilePos> {
        let axial_pos = AxialPos::from(RowOddPos::from(tile_pos));
        let f = |direction| {
            RowOddPos::from(axial_pos.offset(direction)).as_tile_pos_given_map_size(map_size)
        };
        HexNeighbors::from_directional_closure(f)
    }

    /// Returns neighboring tile positions on a map using [`HexCoordSystem::ColEven`].
    ///
    /// A tile position will be `None` for a particular direction, if that neighbor would not lie
    /// on the map.
    pub fn get_neighboring_positions_col_even(
        tile_pos: &TilePos,
        map_size: &TilemapSize,
    ) -> HexNeighbors<TilePos> {
        let axial_pos = AxialPos::from(ColEvenPos::from(tile_pos));
        let f = |direction| {
            ColEvenPos::from(axial_pos.offset(direction)).as_tile_pos_given_map_size(map_size)
        };
        HexNeighbors::from_directional_closure(f)
    }

    /// Returns neighboring tile positions on a map using [`HexCoordSystem::ColOdd`].
    ///
    /// A tile position will be `None` for a particular direction, if that neighbor would not lie
    /// on the map.
    pub fn get_neighboring_positions_col_odd(
        tile_pos: &TilePos,
        map_size: &TilemapSize,
    ) -> HexNeighbors<TilePos> {
        let axial_pos = AxialPos::from(ColOddPos::from(tile_pos));
        let f = |direction| {
            ColOddPos::from(axial_pos.offset(direction)).as_tile_pos_given_map_size(map_size)
        };
        HexNeighbors::from_directional_closure(f)
    }

    /// Returns the entities associated with each tile position.
    pub fn entities(&self, tile_storage: &TileStorage) -> HexNeighbors<Entity> {
        let f = |tile_pos| tile_storage.get(tile_pos);
        self.and_then_ref(f)
    }
}
