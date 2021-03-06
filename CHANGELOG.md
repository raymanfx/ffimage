# Changelog

## TODO
 * Planar API



#### 0.9
> * Packed API rework
>   * Use a single Image struct that is generic over a buffer
>   * Internal matrix representation
> * YUV color support
>   * YUV and YUYV pixel types included

#### 0.8
> * Packed API rework
>   * Renamed traits
>   * Removed AccessPixel traits
>   * Export only generic types in the prelude mod
> * Implemented rectangular sub views for generic images

#### 0.7
> * Dynamic image view/buffer improvements
> * Buffers now offer a from_raw() constructor which consumes a Vec

#### 0.6
> * Benchmark fixes
> * StorageType now derives from num_traits::Num
> * Removed Resize trait

#### 0.5
> * Dynamic image API rework

#### 0.4
> * DynamicImageView and DynamicImageBuffer are now truly dynamic (no more type parameter)!
> * Macropixel support

#### 0.3
> * New DynamicImageBuffer type
> * DynamicImageView no longer stores a format hint
> * Automatic stride calculation and verification

#### 0.2
> * Improved color conversion performance
> * Parallel color conversion using Rayon
> * New DynamicImageView type

#### 0.1
> * Grayscale, Rgb, Rgba, Bgr, Bgra color support
> * Initial release
