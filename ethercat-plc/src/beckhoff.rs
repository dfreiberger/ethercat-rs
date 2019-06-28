// *****************************************************************************
//
// This program is free software; you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation; either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc.,
// 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// Module authors:
//   Georg Brandl <g.brandl@fz-juelich.de>
//
// *****************************************************************************

use ethercat::*;
use ethercat_derive::SlaveProcessImage;
use crate::image::ProcessImage;

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct EK1100 {}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct EL1008 {
    #[entry(0x6000, 1)]  pub input: u8,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
#[pdos(3, Input,  0x1A00, 0x1A01)]
#[pdos(2, Output, 0x1600, 0x1601)]
pub struct EL1502 {
    #[entry(0x1A00, 0x6000, 1)]  pub status_ch1: u16,
    #[entry(0x1A00, 0x6000, 11)] pub value_ch1: u32,
    #[entry(0x1A01, 0x6010, 1)]  pub status_ch2: u16,
    #[entry(0x1A01, 0x6010, 11)] pub value_ch2: u32,

    #[entry(0x1600, 0x7000, 1)]  pub control_ch1: u16,
    #[entry(0x1600, 0x7000, 11)] pub setvalue_ch1: u32,
    #[entry(0x1601, 0x7010, 1)]  pub control_ch2: u16,
    #[entry(0x1601, 0x7010, 11)] pub setvalue_ch2: u32,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct EL1859 {
    #[entry(0x6000, 1)]  pub input: u8,
    #[entry(0x7080, 1)]  pub output: u8,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct EL2008 {
    #[entry(0x7000, 1)]  pub output: u8,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct EL3104 {
    #[entry(0x6000, 1)]  pub ch1_status: u16,
    #[entry(0x6000, 17)] pub ch1: i16,
    #[entry(0x6010, 1)]  pub ch2_status: u16,
    #[entry(0x6010, 17)] pub ch2: i16,
    #[entry(0x6020, 1)]  pub ch3_status: u16,
    #[entry(0x6020, 17)] pub ch3: i16,
    #[entry(0x6030, 1)]  pub ch4_status: u16,
    #[entry(0x6030, 17)] pub ch4: i16,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct EL4132 {
    #[entry(0x3001, 1)]  pub ch1: i16,
    #[entry(0x3002, 1)]  pub ch2: i16,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
#[pdos(3, Input,  0x1A01, 0x1A03, 0x1A04, 0x1A08)]
#[pdos(2, Output, 0x1601, 0x1602, 0x1604)]
pub struct EL7047_Velocity {
    #[entry(0x1A01, 0x6000, 1)]  pub enc_status: u16,
    #[entry(0x1A01, 0x6000, 11)] pub enc_counter: u32,
    #[entry(0x1A01, 0x6000, 12)] pub enc_latch: u32,
    #[entry(0x1A03, 0x6010, 1)]  pub mot_status: u16,
    #[entry(0x1A04, 0x6010, 11)] pub info_data1: u16,
    #[entry(0x1A04, 0x6010, 12)] pub info_data2: u16,
    #[entry(0x1A08, 0x6010, 14)] pub mot_position: i32,

    #[entry(0x1601, 0x7000, 1)]  pub enc_control: u16,
    #[entry(0x1601, 0x7000, 11)] pub enc_set_counter: u32,
    #[entry(0x1602, 0x7010, 1)]  pub mot_control: u16,
    #[entry(0x1604, 0x7010, 21)] pub mot_velocity: i16,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
#[pdos(3, Input,  0x1A01, 0x1A03, 0x1A04, 0x1A08)]
#[pdos(2, Output, 0x1601, 0x1602, 0x1603)]
pub struct EL7047_Position {
    #[entry(0x1A01, 0x6000, 1)]  pub enc_status: u16,
    #[entry(0x1A01, 0x6000, 11)] pub enc_counter: u32,
    #[entry(0x1A01, 0x6000, 12)] pub enc_latch: u32,
    #[entry(0x1A03, 0x6010, 1)]  pub mot_status: u16,
    #[entry(0x1A04, 0x6010, 11)] pub info_data1: u16,
    #[entry(0x1A04, 0x6010, 12)] pub info_data2: u16,
    #[entry(0x1A08, 0x6010, 14)] pub mot_position: i32,

    #[entry(0x1601, 0x7000, 1)]  pub enc_control: u16,
    #[entry(0x1601, 0x7000, 11)] pub enc_set_counter: u32,
    #[entry(0x1602, 0x7010, 1)]  pub mot_control: u16,
    #[entry(0x1603, 0x7010, 11)] pub mot_target: i32,
}

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
#[pdos(3, Input,  0x1A01, 0x1A03, 0x1A07)]
#[pdos(2, Output, 0x1601, 0x1602, 0x1606)]
pub struct EL7047_Positioning {
    #[entry(0x1A01, 0x6000, 1)]  pub enc_status: u16,
    #[entry(0x1A01, 0x6000, 11)] pub enc_counter: u32,
    #[entry(0x1A01, 0x6000, 12)] pub enc_latch: u32,
    #[entry(0x1A03, 0x6010, 1)]  pub mot_status: u16,
    #[entry(0x1A07, 0x6020, 1)]  pub pos_status: u16,
    #[entry(0x1A07, 0x6020, 11)] pub act_pos: i32,
    #[entry(0x1A07, 0x6020, 21)] pub act_velo: u16,
    #[entry(0x1A07, 0x6020, 22)] pub drv_time: u32,

    #[entry(0x1601, 0x7000, 1)]  pub enc_control: u16,
    #[entry(0x1601, 0x7000, 11)] pub enc_set_counter: u32,
    #[entry(0x1602, 0x7010, 1)]  pub mot_control: u16,
    #[entry(0x1606, 0x7020, 1)]  pub pos_control: u16,
    #[entry(0x1606, 0x7020, 11)] pub target_pos: u32,
    #[entry(0x1606, 0x7020, 21)] pub target_velo: u16,
    #[entry(0x1606, 0x7020, 22)] pub start_type: u16,
    #[entry(0x1606, 0x7020, 23)] pub accel: u16,
    #[entry(0x1606, 0x7020, 24)] pub decel: u16,
}
