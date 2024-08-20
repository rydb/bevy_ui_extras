use std::{collections::HashMap, fmt::Display};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;



use bevy_inspector_egui::egui::{Color32, RichText, Ui};
use egui_extras::{Column, Table, TableBuilder, TableRow};
use strum::IntoEnumIterator;

/// Resource for quickly creating a table
/// init this resource with your enum of table attributes, fetch this resource, and use [`.ui`] to render the table.
#[derive(Resource, Default)]
pub struct QuickTable<T>(pub TablePick<T>);

impl<T: IntoEnumIterator + Display + Eq + Copy + Default> QuickTable<T> {
    pub fn ui<'a>(&mut self, ui: &'a mut Ui) -> Table<'a> {
        let collum_count = T::iter().len();
        
        let table = TableBuilder::new(ui)
        .columns(Column::auto()
            //.resizable(true)
            .clip(false)
            //.at_least(150.0)
            , 
            collum_count
        )
        //.min_scrolled_height(0.0)
        .auto_shrink(true)
        .resizable(true)
        //.scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        ;
        table
        .header(20.0, |mut header| {
            self.0.layout_headers(&mut header)
        })
        
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct TablePick<T>(HashMap<String, T>);

impl<T: IntoEnumIterator + Display + Eq + Copy + Default> TablePick<T> {
    /// adds table with options to switch between.
    pub fn get_table(&mut self, ui: &mut Ui) -> &mut Self{
        ui.horizontal(|ui| {
            
            for attr in T::iter() {
                let key = &(attr.to_string());
                let contains_key = (**self).contains_key(key);

                let color = if contains_key {
                    Color32::WHITE
                } else {
                    Color32::GRAY
                };
                if ui.button(
                    RichText::new( attr.to_string())
                    .color(color) 
                )
                .clicked() {
                    if contains_key {
                        (**self).remove(key);

                    } else {
                        (**self).insert(key.clone(), attr);
                    }
                }
            }
        });
        self
    } 
    /// layout headers for table
    pub fn layout_headers(&mut self, ui: &mut TableRow) {
            
        for attr in T::iter() {
            let key = &(attr.to_string());
            let contains_key = (**self).contains_key(key);

            let color = if contains_key {
                Color32::WHITE
            } else {
                Color32::GRAY
            };
            ui.col(|ui| {
                if ui.button(
                    RichText::new( attr.to_string())
                    .color(color) 
                )
                .clicked() {
                    if contains_key {
                        (**self).remove(key);
    
                    } else {
                        (**self).insert(key.clone(), attr);
                    }
                }
            });
            

        }
        //self
    }
}