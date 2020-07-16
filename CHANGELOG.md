# Changelog

## TODO
 * Planar API
 * YUV family color support (possibly in a subcrate)



#### 0.6 (released)
> * Benchmark fixes
> * StorageType now derives from num_traits::Num
> * Removed Resize trait

#### 0.5 (released)
> * Dynamic image API rework

#### 0.4 (released)
> * DynamicImageView and DynamicImageBuffer are now truly dynamic (no more type parameter)!
> * Macropixel support

#### 0.3 (released)
> * New DynamicImageBuffer type
> * DynamicImageView no longer stores a format hint
> * Automatic stride calculation and verification

#### 0.2 (released)
> * Improved color conversion performance
> * Parallel color conversion using Rayon
> * New DynamicImageView type

#### 0.1 (released)
> * Grayscale, Rgb, Rgba, Bgr, Bgra color support
> * Initial release
