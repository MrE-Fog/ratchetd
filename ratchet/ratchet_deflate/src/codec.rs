// Copyright 2015-2021 SWIM.AI inc.
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

use bytes::BytesMut;
use flate2::{
    Compress, CompressError, Decompress, DecompressError, FlushCompress, FlushDecompress, Status,
};
use std::slice;

pub trait BufCompress {
    fn buf_compress(
        &mut self,
        input: &[u8],
        output: &mut BytesMut,
        flush: FlushCompress,
    ) -> Result<Status, CompressError>;
}

pub trait BufDecompress {
    fn buf_decompress(
        &mut self,
        input: &[u8],
        output: &mut BytesMut,
        flush: FlushDecompress,
    ) -> Result<Status, DecompressError>;
}

impl BufCompress for Compress {
    fn buf_compress(
        &mut self,
        input: &[u8],
        output: &mut BytesMut,
        flush: FlushCompress,
    ) -> Result<Status, CompressError> {
        let cap = output.capacity();
        let len = output.len();
        let before = self.total_out();

        unsafe {
            let ptr = output.as_mut_ptr().offset(len as isize);
            let out = slice::from_raw_parts_mut(ptr, cap - len);
            let ret = self.compress(input, out, flush);
            output.set_len((self.total_out() - before) as usize + len);
            ret
        }
    }
}

impl BufDecompress for Decompress {
    fn buf_decompress(
        &mut self,
        input: &[u8],
        output: &mut BytesMut,
        flush: FlushDecompress,
    ) -> Result<Status, DecompressError> {
        let cap = output.capacity();
        let len = output.len();
        let before = self.total_out();

        unsafe {
            let ptr = output.as_mut_ptr().offset(len as isize);
            let out = slice::from_raw_parts_mut(ptr, cap - len);
            let ret = self.decompress(input, out, flush);
            output.set_len((self.total_out() - before) as usize + len);
            ret
        }
    }
}