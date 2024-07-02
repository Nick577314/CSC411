    src = Array2::new.from_rm( img.pixels, img.width, img.height);

    `dest = Array2::new(pixel::new(), img.height, img.width);`

    for( c, r, pix) in src.iter_row_major(){


        // move pix to appropriate coords in dest
        // using dest. get_mut (c2,r2);
    }