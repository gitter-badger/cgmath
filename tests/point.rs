// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


#[macro_use]
extern crate approx;
extern crate cgmath;

use cgmath::Point3;

#[test]
fn test_homogeneous() {
	let p = Point3::new(1.0f64, 2.0f64, 3.0f64);
    assert_ulps_eq!(p, Point3::from_homogeneous(p.to_homogeneous()));
}
