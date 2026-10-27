# Toàn bộ Danh mục Mở rộng: 307 Kỹ xảo Layer EffectCraft (`ec.*`) & 61 Hiệu ứng Âm thanh FilmCraft

Tài liệu này là bảng tra cứu chi tiết cho Mục 7 của `SKILL.md`.

## Phần A — 307 Kỹ xảo Layer After Effects trong EffectCraft (`layer_effects`)

Mọi mã `ec.*` dưới đây đều có thể dùng trực tiếp trong `"motion": {"preset": "<mã ec.*>", "format": "prores"}` hoặc gọi qua `effectcraft-cli --project <file.ecproj> exec effect.apply --params '{"effect":"<mã ec.*>","layers":[1]}' --save`.

| Mã `id` (`ec.*`) | Tên kỹ xảo | Danh mục | Mô tả & Tham số chính |
|---|---|---|---|
| `ec.3d.channelextract` | 3D Channel Extract (ec.3d.channelextract) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · 3D Channel Extract (3D Channel, Black Point, White Point, Clamp Output) |
| `ec.3d.cryptomatte` | Cryptomatte (ec.3d.cryptomatte) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · Cryptomatte (Layer, Selection, Display, Matte Only) |
| `ec.3d.depthmatte` | Depth Matte (ec.3d.depthmatte) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · Depth Matte (Depth, Feather, Invert) |
| `ec.3d.depthoffield` | Depth of Field (ec.3d.depthoffield) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · Depth of Field (Focal Plane, Maximum Radius, Focal Plane Thickness, Focal Bias) |
| `ec.3d.extractor` | EXtractoR (ec.3d.extractor) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · EXtractoR (Red, Green, Blue, Alpha) |
| `ec.3d.fog3d` | Fog 3D (ec.3d.fog3d) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · Fog 3D (Fog Color, Fog Start Depth, Fog End Depth, Fog Opacity) |
| `ec.3d.idmatte` | ID Matte (ec.3d.idmatte) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · ID Matte (Aux. Channel, ID Selection, Feather, Invert) |
| `ec.3d.identifier` | IDentifier (ec.3d.identifier) | AE Layer FX · 3D Channel | EffectCraft 3D Channel · IDentifier (Channel Type, Display, ID) |
| `ec.audio.backwards` | Backwards (ec.audio.backwards) | AE Layer FX · Audio | EffectCraft Audio · Backwards (Swap Channels) |
| `ec.audio.basstreble` | Bass & Treble (ec.audio.basstreble) | AE Layer FX · Audio | EffectCraft Audio · Bass & Treble (Bass, Treble) |
| `ec.audio.compressor` | Compressor (ec.audio.compressor) | AE Layer FX · Audio | EffectCraft Audio · Compressor (Threshold (dB), Ratio (x:1), Knee (dB), Attack (ms)) |
| `ec.audio.delay` | Delay (ec.audio.delay) | AE Layer FX · Audio | EffectCraft Audio · Delay (Delay Time (milliseconds), Delay Amount, Feedback, Dry Out) |
| `ec.audio.distortion` | Distortion (ec.audio.distortion) | AE Layer FX · Audio | EffectCraft Audio · Distortion (Type, Drive, Gain, Mix) |
| `ec.audio.flangechorus` | Flange & Chorus (ec.audio.flangechorus) | AE Layer FX · Audio | EffectCraft Audio · Flange & Chorus (Voice Separation Time (ms), Voices, Modulation Rate, Modulation Depth) |
| `ec.audio.gate` | Gate (ec.audio.gate) | AE Layer FX · Audio | EffectCraft Audio · Gate (Threshold (dB), Attack (ms), Hold (ms), Release (ms)) |
| `ec.audio.highlowpass` | High-Low Pass (ec.audio.highlowpass) | AE Layer FX · Audio | EffectCraft Audio · High-Low Pass (Filter Options, Cutoff Frequency, Dry Out, Wet Out) |
| `ec.audio.modulator` | Modulator (ec.audio.modulator) | AE Layer FX · Audio | EffectCraft Audio · Modulator (Modulation Type, Modulation Rate, Modulation Depth, Amplitude Modulation) |
| `ec.audio.parametriceq` | Parametric EQ (ec.audio.parametriceq) | AE Layer FX · Audio | EffectCraft Audio · Parametric EQ (Band 1 Enabled, Band 1 Frequency, Band 1 Bandwidth, Band 1 Boost/Cut) |
| `ec.audio.reverb` | Reverb (ec.audio.reverb) | AE Layer FX · Audio | EffectCraft Audio · Reverb (Reverb Time (milliseconds), Diffusion, Decay, Brightness) |
| `ec.audio.stereomixer` | Stereo Mixer (ec.audio.stereomixer) | AE Layer FX · Audio | EffectCraft Audio · Stereo Mixer (Left Level, Right Level, Left Pan, Right Pan) |
| `ec.audio.tone` | Tone (ec.audio.tone) | AE Layer FX · Audio | EffectCraft Audio · Tone (Waveform Options, Frequency 1, Frequency 2, Frequency 3) |
| `ec.blur.bilateral` | Bilateral Blur (ec.blur.bilateral) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Bilateral Blur (Radius, Threshold, Colorize) |
| `ec.blur.cccross` | CC Cross Blur (ec.blur.cccross) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · CC Cross Blur (Radius X, Radius Y, Transfer Mode) |
| `ec.blur.ccradial` | CC Radial Blur (ec.blur.ccradial) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · CC Radial Blur (Type, Amount, Quality, Center) |
| `ec.blur.ccradialfast` | CC Radial Fast Blur (ec.blur.ccradialfast) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · CC Radial Fast Blur (Center, Amount, Zoom) |
| `ec.blur.ccvector` | CC Vector Blur (ec.blur.ccvector) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · CC Vector Blur (Type, Amount, Angle Offset, Map Softness) |
| `ec.blur.cameralens` | Camera Lens Blur (ec.blur.cameralens) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Camera Lens Blur (Blur Radius, Shape, Roundness, Aspect Ratio) |
| `ec.blur.camerashakedeblur` | Camera-Shake Deblur (ec.blur.camerashakedeblur) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Camera-Shake Deblur (Blur Duration, Deblur Method, Strength, Shake Sensitivity) |
| `ec.blur.channel` | Channel Blur (ec.blur.channel) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Channel Blur (Red Blurriness, Green Blurriness, Blue Blurriness, Alpha Blurriness) |
| `ec.blur.compound` | Compound Blur (ec.blur.compound) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Compound Blur (Blur Layer, Maximum Blur, Stretch Map to Fit, Invert Blur) |
| `ec.blur.directional` | Directional Blur (ec.blur.directional) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Directional Blur (Direction, Blur Length) |
| `ec.blur.fastbox` | Fast Box Blur (ec.blur.fastbox) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Fast Box Blur (Blur Radius, Iterations, Blur Dimensions, Repeat Edge Pixels) |
| `ec.blur.gaussian` | Gaussian Blur (ec.blur.gaussian) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Gaussian Blur (Blurriness, Blur Dimensions, Repeat Edge Pixels) |
| `ec.blur.radial` | Radial Blur (ec.blur.radial) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Radial Blur (Amount, Center, Type, Antialiasing (Best Quality)) |
| `ec.blur.sharpen` | Sharpen (ec.blur.sharpen) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Sharpen (Sharpen Amount) |
| `ec.blur.smart` | Smart Blur (ec.blur.smart) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Smart Blur (Radius, Threshold, Mode) |
| `ec.blur.unsharp` | Unsharp Mask (ec.blur.unsharp) | AE Layer FX · Blur & Sharpen | EffectCraft Blur & Sharpen · Unsharp Mask (Amount, Radius, Threshold) |
| `ec.channel.arithmetic` | Arithmetic (ec.channel.arithmetic) | AE Layer FX · Channel | EffectCraft Channel · Arithmetic (Operator, Red Value, Green Value, Blue Value) |
| `ec.channel.blend` | Blend (ec.channel.blend) | AE Layer FX · Channel | EffectCraft Channel · Blend (Blend With Layer, Mode, Blend With Original, If Layer Sizes Differ) |
| `ec.channel.cccomposite` | CC Composite (ec.channel.cccomposite) | AE Layer FX · Channel | EffectCraft Channel · CC Composite (Opacity, Composite Original, RGB Only) |
| `ec.channel.calculations` | Calculations (ec.channel.calculations) | AE Layer FX · Channel | EffectCraft Channel · Calculations (Input Channel, Invert Input, Second Layer, Second Layer Channel) |
| `ec.channel.combiner` | Channel Combiner (ec.channel.combiner) | AE Layer FX · Channel | EffectCraft Channel · Channel Combiner (Use 2nd Layer, Source Layer, From, To) |
| `ec.channel.compoundarithmetic` | Compound Arithmetic (ec.channel.compoundarithmetic) | AE Layer FX · Channel | EffectCraft Channel · Compound Arithmetic (Second Source Layer, Operator, Operate On Channels, Overflow Behavior) |
| `ec.channel.invert` | Invert (ec.channel.invert) | AE Layer FX · Channel | EffectCraft Channel · Invert (Channel, Blend With Original) |
| `ec.channel.minimax` | Minimax (ec.channel.minimax) | AE Layer FX · Channel | EffectCraft Channel · Minimax (Operation, Radius, Channel, Direction) |
| `ec.channel.removecolormatting` | Remove Color Matting (ec.channel.removecolormatting) | AE Layer FX · Channel | EffectCraft Channel · Remove Color Matting (Background Color, Clip HDR Values) |
| `ec.channel.setchannels` | Set Channels (ec.channel.setchannels) | AE Layer FX · Channel | EffectCraft Channel · Set Channels (Source Layer 1, Set Red To Source 1's, Source Layer 2, Set Green To Source 2's) |
| `ec.channel.setmatte` | Set Matte (ec.channel.setmatte) | AE Layer FX · Channel | EffectCraft Channel · Set Matte (Take Matte From Layer, Use For Matte, Invert Matte, Stretch Matte to Fit) |
| `ec.channel.shiftchannels` | Shift Channels (ec.channel.shiftchannels) | AE Layer FX · Channel | EffectCraft Channel · Shift Channels (Take Alpha From, Take Red From, Take Green From, Take Blue From) |
| `ec.channel.solidcomposite` | Solid Composite (ec.channel.solidcomposite) | AE Layer FX · Channel | EffectCraft Channel · Solid Composite (Source Opacity, Color, Opacity, Blending Mode) |
| `ec.color.autocolor` | Auto Color (ec.color.autocolor) | AE Layer FX · Color Correction | EffectCraft Color Correction · Auto Color (Temporal Smoothing (seconds), Scene Detect, Black Clip, White Clip) |
| `ec.color.autocontrast` | Auto Contrast (ec.color.autocontrast) | AE Layer FX · Color Correction | EffectCraft Color Correction · Auto Contrast (Temporal Smoothing (seconds), Scene Detect, Black Clip, White Clip) |
| `ec.color.autolevels` | Auto Levels (ec.color.autolevels) | AE Layer FX · Color Correction | EffectCraft Color Correction · Auto Levels (Temporal Smoothing (seconds), Scene Detect, Black Clip, White Clip) |
| `ec.color.blackwhite` | Black & White (ec.color.blackwhite) | AE Layer FX · Color Correction | EffectCraft Color Correction · Black & White (Reds, Yellows, Greens, Cyans) |
| `ec.color.brightnesscontrast` | Brightness & Contrast (ec.color.brightnesscontrast) | AE Layer FX · Color Correction | EffectCraft Color Correction · Brightness & Contrast (Brightness, Contrast, Use Legacy) |
| `ec.color.broadcast` | Broadcast Colors (ec.color.broadcast) | AE Layer FX · Color Correction | EffectCraft Color Correction · Broadcast Colors (Broadcast Locale, How To Make Color Safe, Maximum Signal Amplitude (IRE)) |
| `ec.color.cccolorneutralizer` | CC Color Neutralizer (ec.color.cccolorneutralizer) | AE Layer FX · Color Correction | EffectCraft Color Correction · CC Color Neutralizer (Shadows Unbalance, Shadows Balance, Midtones Unbalance, Midtones Balance) |
| `ec.color.cccoloroffset` | CC Color Offset (ec.color.cccoloroffset) | AE Layer FX · Color Correction | EffectCraft Color Correction · CC Color Offset (Red Phase, Green Phase, Blue Phase, Overflow) |
| `ec.color.cckernel` | CC Kernel (ec.color.cckernel) | AE Layer FX · Color Correction | EffectCraft Color Correction · CC Kernel (Top Left, Top, Top Right, Left) |
| `ec.color.cctoner` | CC Toner (ec.color.cctoner) | AE Layer FX · Color Correction | EffectCraft Color Correction · CC Toner (Tones, Highlights, Brights, Midtones) |
| `ec.color.changecolor` | Change Color (ec.color.changecolor) | AE Layer FX · Color Correction | EffectCraft Color Correction · Change Color (View, Hue Transform, Lightness Transform, Saturation Transform) |
| `ec.color.changetocolor` | Change to Color (ec.color.changetocolor) | AE Layer FX · Color Correction | EffectCraft Color Correction · Change to Color (From, To, Change, Change By) |
| `ec.color.channelmixer` | Channel Mixer (ec.color.channelmixer) | AE Layer FX · Color Correction | EffectCraft Color Correction · Channel Mixer (Red-Red, Red-Green, Red-Blue, Red-Const) |
| `ec.color.colorbalance` | Color Balance (ec.color.colorbalance) | AE Layer FX · Color Correction | EffectCraft Color Correction · Color Balance (Shadow Red Balance, Shadow Green Balance, Shadow Blue Balance, Midtone Red Balance) |
| `ec.color.colorbalancehls` | Color Balance (HLS) (ec.color.colorbalancehls) | AE Layer FX · Color Correction | EffectCraft Color Correction · Color Balance (HLS) (Hue, Lightness, Saturation) |
| `ec.color.colorlink` | Color Link (ec.color.colorlink) | AE Layer FX · Color Correction | EffectCraft Color Correction · Color Link (Source Layer, Source Layer Source, Sample, Clip) |
| `ec.color.colorstabilizer` | Color Stabilizer (ec.color.colorstabilizer) | AE Layer FX · Color Correction | EffectCraft Color Correction · Color Stabilizer (Reference Frame (s), Stabilize, Black Point, Mid Point) |
| `ec.color.colorama` | Colorama (ec.color.colorama) | AE Layer FX · Color Correction | EffectCraft Color Correction · Colorama (Get Phase From, Add Phase, Add Phase From, Add Mode) |
| `ec.color.curves` | Curves (ec.color.curves) | AE Layer FX · Color Correction | EffectCraft Color Correction · Curves (Channel, RGB, Red, Green) |
| `ec.color.equalize` | Equalize (ec.color.equalize) | AE Layer FX · Color Correction | EffectCraft Color Correction · Equalize (Equalize, Amount to Equalize) |
| `ec.color.exposure` | Exposure (ec.color.exposure) | AE Layer FX · Color Correction | EffectCraft Color Correction · Exposure (Channels, Exposure, Offset, Gamma Correction) |
| `ec.color.gammapedestalgain` | Gamma/Pedestal/Gain (ec.color.gammapedestalgain) | AE Layer FX · Color Correction | EffectCraft Color Correction · Gamma/Pedestal/Gain (Black Stretch, Red Gamma, Red Pedestal, Red Gain) |
| `ec.color.huesaturation` | Hue/Saturation (ec.color.huesaturation) | AE Layer FX · Color Correction | EffectCraft Color Correction · Hue/Saturation (Channel Control, Master Hue, Master Saturation, Master Lightness) |
| `ec.color.leavecolor` | Leave Color (ec.color.leavecolor) | AE Layer FX · Color Correction | EffectCraft Color Correction · Leave Color (Amount to Decolor, Color To Leave, Tolerance, Edge Softness) |
| `ec.color.levels` | Levels (ec.color.levels) | AE Layer FX · Color Correction | EffectCraft Color Correction · Levels (Channel, Input Black, Input White, Gamma) |
| `ec.color.levelsic` | Levels (Individual Controls) (ec.color.levelsic) | AE Layer FX · Color Correction | EffectCraft Color Correction · Levels (Individual Controls) (Input Black, Input White, Gamma, Output Black) |
| `ec.color.lumetri` | Lumetri Color (ec.color.lumetri) | AE Layer FX · Color Correction | EffectCraft Color Correction · Lumetri Color (High Dynamic Range, Input LUT, Input LUT File, Temperature) |
| `ec.color.ociocdl` | OCIO CDL Transform (ec.color.ociocdl) | AE Layer FX · Color Correction | EffectCraft Color Correction · OCIO CDL Transform (Slope Red, Slope Green, Slope Blue, Offset Red) |
| `ec.color.ociocolorspace` | OCIO Color Space Transform (ec.color.ociocolorspace) | AE Layer FX · Color Correction | EffectCraft Color Correction · OCIO Color Space Transform (Configuration, Config File (.ocio), Source, Source (config name)) |
| `ec.color.ociodisplay` | OCIO Display Transform (ec.color.ociodisplay) | AE Layer FX · Color Correction | EffectCraft Color Correction · OCIO Display Transform (Configuration, Config File (.ocio), Input Color Space, Source (config name)) |
| `ec.color.ociofile` | OCIO File Transform (ec.color.ociofile) | AE Layer FX · Color Correction | EffectCraft Color Correction · OCIO File Transform (File, CCC ID, Interpolation, Direction) |
| `ec.color.ociolook` | OCIO Look Transform (ec.color.ociolook) | AE Layer FX · Color Correction | EffectCraft Color Correction · OCIO Look Transform (Configuration, Source, Destination, Look) |
| `ec.color.psarbitrarymap` | PS Arbitrary Map (ec.color.psarbitrarymap) | AE Layer FX · Color Correction | EffectCraft Color Correction · PS Arbitrary Map (Phase, Apply Phase Map To Alpha, Map) |
| `ec.color.photofilter` | Photo Filter (ec.color.photofilter) | AE Layer FX · Color Correction | EffectCraft Color Correction · Photo Filter (Filter, Color, Density, Preserve Luminosity) |
| `ec.color.selectivecolor` | Selective Color (ec.color.selectivecolor) | AE Layer FX · Color Correction | EffectCraft Color Correction · Selective Color (Method, Colors, Reds Cyan, Reds Magenta) |
| `ec.color.shadowhighlight` | Shadow/Highlight (ec.color.shadowhighlight) | AE Layer FX · Color Correction | EffectCraft Color Correction · Shadow/Highlight (Auto Amounts, Shadow Amount, Highlight Amount, Temporal Smoothing (seconds)) |
| `ec.color.tint` | Tint (ec.color.tint) | AE Layer FX · Color Correction | EffectCraft Color Correction · Tint (Map Black To, Map White To, Amount to Tint) |
| `ec.color.tritone` | Tritone (ec.color.tritone) | AE Layer FX · Color Correction | EffectCraft Color Correction · Tritone (Highlights, Midtones, Shadows, Blend With Original) |
| `ec.color.vibrance` | Vibrance (ec.color.vibrance) | AE Layer FX · Color Correction | EffectCraft Color Correction · Vibrance (Vibrance, Saturation) |
| `ec.color.videolimiter` | Video Limiter (ec.color.videolimiter) | AE Layer FX · Color Correction | EffectCraft Color Correction · Video Limiter (Clip Level, Clip Method, Compression before Clipping, Gamut Warning) |
| `ec.distort.bezierwarp` | Bezier Warp (ec.distort.bezierwarp) | AE Layer FX · Distort | EffectCraft Distort · Bezier Warp (Top Left Vertex, Top Left Tangent, Top Right Tangent, Right Top Vertex) |
| `ec.distort.bulge` | Bulge (ec.distort.bulge) | AE Layer FX · Distort | EffectCraft Distort · Bulge (Horizontal Radius, Vertical Radius, Bulge Center, Bulge Height) |
| `ec.distort.ccbendit` | CC Bend It (ec.distort.ccbendit) | AE Layer FX · Distort | EffectCraft Distort · CC Bend It (Bend, Start, End, Render Prestart) |
| `ec.distort.ccbender` | CC Bender (ec.distort.ccbender) | AE Layer FX · Distort | EffectCraft Distort · CC Bender (Amount, Style, Adjust To Distance, Top) |
| `ec.distort.ccblobbylize` | CC Blobbylize (ec.distort.ccblobbylize) | AE Layer FX · Distort | EffectCraft Distort · CC Blobbylize (Blob Layer, Property, Softness, Cut Away) |
| `ec.distort.ccflomotion` | CC Flo Motion (ec.distort.ccflomotion) | AE Layer FX · Distort | EffectCraft Distort · CC Flo Motion (Knot 1, Amount 1, Knot 2, Amount 2) |
| `ec.distort.ccgriddler` | CC Griddler (ec.distort.ccgriddler) | AE Layer FX · Distort | EffectCraft Distort · CC Griddler (Horizontal Scale, Vertical Scale, Tile Size, Rotation) |
| `ec.distort.cclens` | CC Lens (ec.distort.cclens) | AE Layer FX · Distort | EffectCraft Distort · CC Lens (Center, Size, Convergence) |
| `ec.distort.ccpageturn` | CC Page Turn (ec.distort.ccpageturn) | AE Layer FX · Distort | EffectCraft Distort · CC Page Turn (Fold Position, Fold Direction, Fold Radius, Light Direction) |
| `ec.distort.ccpowerpin` | CC Power Pin (ec.distort.ccpowerpin) | AE Layer FX · Distort | EffectCraft Distort · CC Power Pin (Top Left, Top Right, Bottom Left, Bottom Right) |
| `ec.distort.ccripplepulse` | CC Ripple Pulse (ec.distort.ccripplepulse) | AE Layer FX · Distort | EffectCraft Distort · CC Ripple Pulse (Center, Pulse Level (animate), Time Span (sec), Amplitude) |
| `ec.distort.ccslant` | CC Slant (ec.distort.ccslant) | AE Layer FX · Distort | EffectCraft Distort · CC Slant (Slant, Stretching, Height, Floor) |
| `ec.distort.ccsmear` | CC Smear (ec.distort.ccsmear) | AE Layer FX · Distort | EffectCraft Distort · CC Smear (From, To, Reach, Radius) |
| `ec.distort.ccsplit` | CC Split (ec.distort.ccsplit) | AE Layer FX · Distort | EffectCraft Distort · CC Split (Point A, Point B, Split) |
| `ec.distort.ccsplit2` | CC Split 2 (ec.distort.ccsplit2) | AE Layer FX · Distort | EffectCraft Distort · CC Split 2 (Point A, Point B, Split 1, Split 2) |
| `ec.distort.cctiler` | CC Tiler (ec.distort.cctiler) | AE Layer FX · Distort | EffectCraft Distort · CC Tiler (Scale, Center, Blend w. Original) |
| `ec.distort.cornerpin` | Corner Pin (ec.distort.cornerpin) | AE Layer FX · Distort | EffectCraft Distort · Corner Pin (Upper Left, Upper Right, Lower Left, Lower Right) |
| `ec.distort.upscale` | Detail-preserving Upscale (ec.distort.upscale) | AE Layer FX · Distort | EffectCraft Distort · Detail-preserving Upscale (Scale, Reduce Noise, Detail, Alpha) |
| `ec.distort.displacementmap` | Displacement Map (ec.distort.displacementmap) | AE Layer FX · Distort | EffectCraft Distort · Displacement Map (Displacement Map Layer, Displacement Map Layer Source, Use For Horizontal Displacement, Max Horizontal Displacement) |
| `ec.distort.liquify` | Liquify (ec.distort.liquify) | AE Layer FX · Distort | EffectCraft Distort · Liquify (Tool, Brush Size, Brush Pressure, Freeze Area Mask) |
| `ec.distort.magnify` | Magnify (ec.distort.magnify) | AE Layer FX · Distort | EffectCraft Distort · Magnify (Shape, Center, Magnification, Link) |
| `ec.distort.meshwarp` | Mesh Warp (ec.distort.meshwarp) | AE Layer FX · Distort | EffectCraft Distort · Mesh Warp (Rows, Columns, Quality, Distortion Mesh) |
| `ec.distort.mirror` | Mirror (ec.distort.mirror) | AE Layer FX · Distort | EffectCraft Distort · Mirror (Reflection Center, Reflection Angle) |
| `ec.distort.offset` | Offset (ec.distort.offset) | AE Layer FX · Distort | EffectCraft Distort · Offset (Shift Center To, Blend With Original) |
| `ec.distort.opticscompensation` | Optics Compensation (ec.distort.opticscompensation) | AE Layer FX · Distort | EffectCraft Distort · Optics Compensation (Field Of View (FOV), Reverse Lens Distortion, FOV Orientation, View Center) |
| `ec.distort.polar` | Polar Coordinates (ec.distort.polar) | AE Layer FX · Distort | EffectCraft Distort · Polar Coordinates (Interpolation, Type of Conversion) |
| `ec.distort.puppet` | Puppet (ec.distort.puppet) | AE Layer FX · Distort | EffectCraft Distort · Puppet (Mesh Rotation Refinement) |
| `ec.distort.reshape` | Reshape (ec.distort.reshape) | AE Layer FX · Distort | EffectCraft Distort · Reshape (Source Mask, Destination Mask, Boundary Mask, Percent) |
| `ec.distort.ripple` | Ripple (ec.distort.ripple) | AE Layer FX · Distort | EffectCraft Distort · Ripple (Radius, Center of Ripple, Type of Conversion, Wave Speed) |
| `ec.distort.rollingshutterrepair` | Rolling Shutter Repair (ec.distort.rollingshutterrepair) | AE Layer FX · Distort | EffectCraft Distort · Rolling Shutter Repair (Rolling Shutter Rate, Scan Direction, Method, Detailed Analysis) |
| `ec.distort.smear` | Smear (ec.distort.smear) | AE Layer FX · Distort | EffectCraft Distort · Smear (Source Mask, Boundary Mask, Mask Offset, Mask Rotation) |
| `ec.distort.spherize` | Spherize (ec.distort.spherize) | AE Layer FX · Distort | EffectCraft Distort · Spherize (Radius, Center of Sphere) |
| `ec.distort.transform` | Transform (ec.distort.transform) | AE Layer FX · Distort | EffectCraft Distort · Transform (Anchor Point, Position, Uniform Scale, Scale Height) |
| `ec.distort.turbulentdisplace` | Turbulent Displace (ec.distort.turbulentdisplace) | AE Layer FX · Distort | EffectCraft Distort · Turbulent Displace (Displacement, Amount, Size, Offset (Turbulence)) |
| `ec.distort.twirl` | Twirl (ec.distort.twirl) | AE Layer FX · Distort | EffectCraft Distort · Twirl (Angle, Twirl Radius, Twirl Center) |
| `ec.distort.twirllegacy` | Twirl (Legacy) (ec.distort.twirllegacy) | AE Layer FX · Distort | EffectCraft Distort · Twirl (Legacy) (Angle, Twirl Radius, Twirl Center) |
| `ec.distort.warp` | Warp (ec.distort.warp) | AE Layer FX · Distort | EffectCraft Distort · Warp (Warp Style, Warp Axis, Bend, Horizontal Distortion) |
| `ec.distort.warpstabilizer` | Warp Stabilizer (ec.distort.warpstabilizer) | AE Layer FX · Distort | EffectCraft Distort · Warp Stabilizer (Result, Smoothness, Method, Preserve Scale) |
| `ec.distort.wavewarp` | Wave Warp (ec.distort.wavewarp) | AE Layer FX · Distort | EffectCraft Distort · Wave Warp (Wave Type, Wave Height, Wave Width, Direction) |
| `ec.control.point3d` | 3D Point Control (ec.control.point3d) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · 3D Point Control (3D Point) |
| `ec.control.angle` | Angle Control (ec.control.angle) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Angle Control (Angle) |
| `ec.control.checkbox` | Checkbox Control (ec.control.checkbox) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Checkbox Control (Checkbox) |
| `ec.control.color` | Color Control (ec.control.color) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Color Control (Color) |
| `ec.control.dropdown` | Dropdown Menu Control (ec.control.dropdown) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Dropdown Menu Control (Menu) |
| `ec.control.layer` | Layer Control (ec.control.layer) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Layer Control (Layer) |
| `ec.control.point` | Point Control (ec.control.point) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Point Control (Point) |
| `ec.control.slider` | Slider Control (ec.control.slider) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Slider Control (Slider) |
| `ec.control.stereo3d` | Stereo 3D Controls (ec.control.stereo3d) | AE Layer FX · Expression Controls | EffectCraft Expression Controls · Stereo 3D Controls (Configuration, Stereo Scene Depth, Enable Convergence, Convergence Of) |
| `ec.generate.fourcolor` | 4-Color Gradient (ec.generate.fourcolor) | AE Layer FX · Generate | EffectCraft Generate · 4-Color Gradient (Point 1, Color 1, Point 2, Color 2) |
| `ec.generate.advancedlightning` | Advanced Lightning (ec.generate.advancedlightning) | AE Layer FX · Generate | EffectCraft Generate · Advanced Lightning (Lightning Type, Origin, Direction, Conductivity State) |
| `ec.generate.audiospectrum` | Audio Spectrum (ec.generate.audiospectrum) | AE Layer FX · Generate | EffectCraft Generate · Audio Spectrum (Audio Layer, Start Point, End Point, Path) |
| `ec.generate.audiowaveform` | Audio Waveform (ec.generate.audiowaveform) | AE Layer FX · Generate | EffectCraft Generate · Audio Waveform (Audio Layer, Start Point, End Point, Path) |
| `ec.generate.beam` | Beam (ec.generate.beam) | AE Layer FX · Generate | EffectCraft Generate · Beam (Starting Point, Ending Point, Length, Time) |
| `ec.generate.ccgluegun` | CC Glue Gun (ec.generate.ccgluegun) | AE Layer FX · Generate | EffectCraft Generate · CC Glue Gun (Brush Position, Stroke Width, Density, Time Span (sec)) |
| `ec.generate.cclightburst` | CC Light Burst 2.5 (ec.generate.cclightburst) | AE Layer FX · Generate | EffectCraft Generate · CC Light Burst 2.5 (Center, Intensity, Ray Length, Burst) |
| `ec.generate.cclightrays` | CC Light Rays (ec.generate.cclightrays) | AE Layer FX · Generate | EffectCraft Generate · CC Light Rays (Intensity, Center, Radius, Warp Softness) |
| `ec.generate.cclightsweep` | CC Light Sweep (ec.generate.cclightsweep) | AE Layer FX · Generate | EffectCraft Generate · CC Light Sweep (Center, Direction, Shape, Width) |
| `ec.generate.ccthreads` | CC Threads (ec.generate.ccthreads) | AE Layer FX · Generate | EffectCraft Generate · CC Threads (Width, Height, Overlaps, Direction) |
| `ec.generate.cellpattern` | Cell Pattern (ec.generate.cellpattern) | AE Layer FX · Generate | EffectCraft Generate · Cell Pattern (Cell Pattern, Invert, Contrast, Overflow) |
| `ec.generate.checkerboard` | Checkerboard (ec.generate.checkerboard) | AE Layer FX · Generate | EffectCraft Generate · Checkerboard (Anchor, Size From, Corner, Width) |
| `ec.generate.circle` | Circle (ec.generate.circle) | AE Layer FX · Generate | EffectCraft Generate · Circle (Center, Radius, Edge, Edge Radius) |
| `ec.generate.ellipse` | Ellipse (ec.generate.ellipse) | AE Layer FX · Generate | EffectCraft Generate · Ellipse (Center, Width, Height, Thickness) |
| `ec.generate.eyedropperfill` | Eyedropper Fill (ec.generate.eyedropperfill) | AE Layer FX · Generate | EffectCraft Generate · Eyedropper Fill (Sample Point, Sample Radius, Average Pixel Colors, Maintain Original Alpha) |
| `ec.generate.fill` | Fill (ec.generate.fill) | AE Layer FX · Generate | EffectCraft Generate · Fill (Fill Mask, All Masks, Color, Invert) |
| `ec.generate.fractal` | Fractal (ec.generate.fractal) | AE Layer FX · Generate | EffectCraft Generate · Fractal (Set Choice, Equation, X (Real), Y (Imaginary)) |
| `ec.generate.gradientramp` | Gradient Ramp (ec.generate.gradientramp) | AE Layer FX · Generate | EffectCraft Generate · Gradient Ramp (Start of Ramp, Start Color, End of Ramp, End Color) |
| `ec.generate.grid` | Grid (ec.generate.grid) | AE Layer FX · Generate | EffectCraft Generate · Grid (Anchor, Size From, Corner, Width) |
| `ec.generate.lensflare` | Lens Flare (ec.generate.lensflare) | AE Layer FX · Generate | EffectCraft Generate · Lens Flare (Flare Center, Flare Brightness, Lens Type, Blend With Original) |
| `ec.generate.paintbucket` | Paint Bucket (ec.generate.paintbucket) | AE Layer FX · Generate | EffectCraft Generate · Paint Bucket (Fill Point, Fill Selector, Tolerance, View Threshold) |
| `ec.generate.radiowaves` | Radio Waves (ec.generate.radiowaves) | AE Layer FX · Generate | EffectCraft Generate · Radio Waves (Producer Point, Parameters Are Set At, Render Quality, Wave Type) |
| `ec.generate.scribble` | Scribble (ec.generate.scribble) | AE Layer FX · Generate | EffectCraft Generate · Scribble (Scribble, Mask, Fill Type, Edge Width) |
| `ec.generate.stroke` | Stroke (ec.generate.stroke) | AE Layer FX · Generate | EffectCraft Generate · Stroke (Path, All Masks, Stroke Sequentially, Color) |
| `ec.generate.vegas` | Vegas (ec.generate.vegas) | AE Layer FX · Generate | EffectCraft Generate · Vegas (Stroke, Image Contours: Input Layer, Image Contours: Invert Input, Image Contours: If Layer Sizes Differ) |
| `ec.generate.writeon` | Write-on (ec.generate.writeon) | AE Layer FX · Generate | EffectCraft Generate · Write-on (Brush Position, Color, Brush Size, Brush Hardness) |
| `ec.vr.blur` | VR Blur (ec.vr.blur) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Blur (Frame Layout, Blurriness) |
| `ec.vr.chromaticaberrations` | VR Chromatic Aberrations (ec.vr.chromaticaberrations) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Chromatic Aberrations (Frame Layout, Center Tilt, Center Pan, Aberration (Red)) |
| `ec.vr.colorgradients` | VR Color Gradients (ec.vr.colorgradients) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Color Gradients (Frame Layout, Enable Point 1, Point 1, Color 1) |
| `ec.vr.converter` | VR Converter (ec.vr.converter) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Converter (Frame Layout, Source Projection, Source Horizontal FOV, Target Projection) |
| `ec.vr.denoise` | VR De-Noise (ec.vr.denoise) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR De-Noise (Frame Layout, Noise Type, Noise Level, Detail) |
| `ec.vr.digitalglitch` | VR Digital Glitch (ec.vr.digitalglitch) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Digital Glitch (Frame Layout, Master Amplitude, Distortion Rate, Distortion Complexity) |
| `ec.vr.fractalnoise` | VR Fractal Noise (ec.vr.fractalnoise) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Fractal Noise (Frame Layout, Fractal Type, Contrast, Brightness) |
| `ec.vr.glow` | VR Glow (ec.vr.glow) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Glow (Frame Layout, Luma Threshold, Glow Radius, Glow Brightness) |
| `ec.vr.planetosphere` | VR Plane to Sphere (ec.vr.planetosphere) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Plane to Sphere (Frame Layout, Scale (Degrees), Feather, Tilt (X Axis)) |
| `ec.vr.rotatesphere` | VR Rotate Sphere (ec.vr.rotatesphere) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Rotate Sphere (Frame Layout, Invert Rotation, Tilt (X Axis), Pan (Y Axis)) |
| `ec.vr.sharpen` | VR Sharpen (ec.vr.sharpen) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Sharpen (Frame Layout, Sharpen Amount) |
| `ec.vr.spheretoplane` | VR Sphere to Plane (ec.vr.spheretoplane) | AE Layer FX · Immersive Video | EffectCraft Immersive Video · VR Sphere to Plane (Frame Layout, Horizontal FOV, Tilt (X Axis), Pan (Y Axis)) |
| `ec.key.advancedspill` | Advanced Spill Suppressor (ec.key.advancedspill) | AE Layer FX · Keying | EffectCraft Keying · Advanced Spill Suppressor (Method, Suppression, Key Color, Tolerance) |
| `ec.key.ccsimplewireremoval` | CC Simple Wire Removal (ec.key.ccsimplewireremoval) | AE Layer FX · Keying | EffectCraft Keying · CC Simple Wire Removal (Point A, Point B, Removal Style, Thickness) |
| `ec.key.colordifference` | Color Difference Key (ec.key.colordifference) | AE Layer FX · Keying | EffectCraft Keying · Color Difference Key (View, Key Color, Color Matching Accuracy, Partial A In Black) |
| `ec.key.colorrange` | Color Range (ec.key.colorrange) | AE Layer FX · Keying | EffectCraft Keying · Color Range (Fuzziness, Color Space, Min (L, Y, R), Max (L, Y, R)) |
| `ec.key.differencematte` | Difference Matte (ec.key.differencematte) | AE Layer FX · Keying | EffectCraft Keying · Difference Matte (View, Difference Layer, If Layer Sizes Differ, Matching Tolerance) |
| `ec.key.extract` | Extract (ec.key.extract) | AE Layer FX · Keying | EffectCraft Keying · Extract (Channel, Black Point, White Point, Black Softness) |
| `ec.key.innerouter` | Inner/Outer Key (ec.key.innerouter) | AE Layer FX · Keying | EffectCraft Keying · Inner/Outer Key (Foreground (Inside), Foreground 1, Foreground 2, Foreground 3) |
| `ec.key.keycleaner` | Key Cleaner (ec.key.keycleaner) | AE Layer FX · Keying | EffectCraft Keying · Key Cleaner (Additional Edge Radius, Reduce Chatter, Alpha Contrast, Strength) |
| `ec.keying.keylight` | Key Light (ec.keying.keylight) | AE Layer FX · Keying | EffectCraft Keying · Key Light (View, Unpremultiply Result, Screen Colour, Screen Gain) |
| `ec.key.linearcolor` | Linear Color Key (ec.key.linearcolor) | AE Layer FX · Keying | EffectCraft Keying · Linear Color Key (View, Key Color, Match colors, Matching Tolerance) |
| `ec.key.screen` | Screen Key (ec.key.screen) | AE Layer FX · Keying | EffectCraft Keying · Screen Key (View, Screen Colour, Screen Gain, Screen Balance) |
| `ec.key.unmult` | Unmult (ec.key.unmult) | AE Layer FX · Keying | EffectCraft Keying · Unmult (Background Color, Black Level, White Level, Softness) |
| `ec.matte.mattechoker` | Matte Choker (ec.matte.mattechoker) | AE Layer FX · Matte | EffectCraft Matte · Matte Choker (Geometric Softness 1, Choke 1, Gray Level Softness 1, Geometric Softness 2) |
| `ec.obsolete.mochashape` | Mocha shape (ec.obsolete.mochashape) | AE Layer FX · Matte | EffectCraft Matte · Mocha shape (Blend mode, Invert, Render edge width, Render type) |
| `ec.matte.objectmatte` | Object Matte (ec.matte.objectmatte) | AE Layer FX · Matte | EffectCraft Matte · Object Matte (Object Point, Use Box, Box Top Left, Box Bottom Right) |
| `ec.matte.refinehard` | Refine Hard Matte (ec.matte.refinehard) | AE Layer FX · Matte | EffectCraft Matte · Refine Hard Matte (Smooth, Feather, Choke, Reduce Chatter) |
| `ec.matte.refinesoft` | Refine Soft Matte (ec.matte.refinesoft) | AE Layer FX · Matte | EffectCraft Matte · Refine Soft Matte (Calculate Edge Details, Additional Edge Radius, View Edge Region, Smooth) |
| `ec.matte.rotobrush` | Roto Brush & Refine Edge (ec.matte.rotobrush) | AE Layer FX · Matte | EffectCraft Matte · Roto Brush & Refine Edge (Version, Quality, Search Radius, Motion Threshold) |
| `ec.matte.simplechoker` | Simple Choker (ec.matte.simplechoker) | AE Layer FX · Matte | EffectCraft Matte · Simple Choker (View, Choke Matte) |
| `ec.noise.addgrain` | Add Grain (ec.noise.addgrain) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Add Grain (Viewing Mode, Preset, Center, Width) |
| `ec.noise.curlnoise` | Curl Noise (ec.noise.curlnoise) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Curl Noise (View, Displacement Amount, Scale, Rotation) |
| `ec.noise.dustscratches` | Dust & Scratches (ec.noise.dustscratches) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Dust & Scratches (Radius, Threshold, Operate On Alpha Channel) |
| `ec.noise.fractal` | Fractal Noise (ec.noise.fractal) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Fractal Noise (Fractal Type, Noise Type, Invert, Contrast) |
| `ec.noise.matchgrain` | Match Grain (ec.noise.matchgrain) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Match Grain (Viewing Mode, Noise Source Layer, Noise Source Layer Source, Compensate for Existing Noise) |
| `ec.noise.median` | Median (ec.noise.median) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Median (Radius, Operate On Alpha Channel) |
| `ec.noise.medianlegacy` | Median (Legacy) (ec.noise.medianlegacy) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Median (Legacy) (Radius, Operate On Alpha Channel) |
| `ec.noise.noise` | Noise (ec.noise.noise) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Noise (Amount of Noise, Use Color Noise, Clip Result Values) |
| `ec.noise.noisealpha` | Noise Alpha (ec.noise.noisealpha) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Noise Alpha (Noise, Amount, Original Alpha, Overflow) |
| `ec.noise.noisehls` | Noise HLS (ec.noise.noisehls) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Noise HLS (Noise, Hue, Lightness, Saturation) |
| `ec.noise.noisehlsauto` | Noise HLS Auto (ec.noise.noisehlsauto) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Noise HLS Auto (Noise, Hue, Lightness, Saturation) |
| `ec.noise.removegrain` | Remove Grain (ec.noise.removegrain) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Remove Grain (Viewing Mode, Center, Width, Height) |
| `ec.noise.turbulent` | Turbulent Noise (ec.noise.turbulent) | AE Layer FX · Noise & Grain | EffectCraft Noise & Grain · Turbulent Noise (Fractal Type, Noise Type, Invert, Contrast) |
| `ec.obsolete.basic3d` | Basic 3D (ec.obsolete.basic3d) | AE Layer FX · Obsolete | EffectCraft Obsolete · Basic 3D (Swivel, Tilt, Distance to Image, Specular Highlight) |
| `ec.obsolete.basictext` | Basic Text (ec.obsolete.basictext) | AE Layer FX · Obsolete | EffectCraft Obsolete · Basic Text (Text, Alignment, Position, Display Options) |
| `ec.key.colorkey` | Color Key (ec.key.colorkey) | AE Layer FX · Obsolete | EffectCraft Obsolete · Color Key (Key Color, Color Tolerance, Edge Thin, Edge Feather) |
| `ec.obsolete.gaussianlegacy` | Gaussian Blur (Legacy) (ec.obsolete.gaussianlegacy) | AE Layer FX · Obsolete | EffectCraft Obsolete · Gaussian Blur (Legacy) (Blurriness, Blur Dimensions) |
| `ec.obsolete.lightning` | Lightning (ec.obsolete.lightning) | AE Layer FX · Obsolete | EffectCraft Obsolete · Lightning (Start Point, End Point, Segments, Amplitude) |
| `ec.key.luma` | Luma Key (ec.key.luma) | AE Layer FX · Obsolete | EffectCraft Obsolete · Luma Key (Key Type, Threshold, Tolerance, Edge Thin) |
| `ec.obsolete.pathtext` | Path Text (ec.obsolete.pathtext) | AE Layer FX · Obsolete | EffectCraft Obsolete · Path Text (Text, Shape Type, Tangent 1/Circle Point, Vertex 1/Circle Center) |
| `ec.blur.reduceflicker` | Reduce Interlace Flicker (ec.blur.reduceflicker) | AE Layer FX · Obsolete | EffectCraft Obsolete · Reduce Interlace Flicker (Softness) |
| `ec.key.spill` | Spill Suppressor (ec.key.spill) | AE Layer FX · Obsolete | EffectCraft Obsolete · Spill Suppressor (Color To Suppress, Color Accuracy, Suppression) |
| `ec.paint.paint` | Paint (ec.paint.paint) | AE Layer FX · Paint | EffectCraft Paint · Paint (Paint on Transparent) |
| `ec.perspective.cameratracker` | 3D Camera Tracker (ec.perspective.cameratracker) | AE Layer FX · Perspective | EffectCraft Perspective · 3D Camera Tracker (Shot Type, Horizontal Angle of View, Show Track Points, Render Track Points) |
| `ec.perspective.3dglasses` | 3D Glasses (ec.perspective.3dglasses) | AE Layer FX · Perspective | EffectCraft Perspective · 3D Glasses (Left View, Right View, Scene Convergence, Vertical Alignment) |
| `ec.perspective.bevelalpha` | Bevel Alpha (ec.perspective.bevelalpha) | AE Layer FX · Perspective | EffectCraft Perspective · Bevel Alpha (Edge Thickness, Light Angle, Light Color, Light Intensity) |
| `ec.perspective.beveledges` | Bevel Edges (ec.perspective.beveledges) | AE Layer FX · Perspective | EffectCraft Perspective · Bevel Edges (Edge Thickness, Light Angle, Light Color, Light Intensity) |
| `ec.perspective.cccylinder` | CC Cylinder (ec.perspective.cccylinder) | AE Layer FX · Perspective | EffectCraft Perspective · CC Cylinder (Radius, Position, Rotation, Rotation X) |
| `ec.perspective.ccenvironment` | CC Environment (ec.perspective.ccenvironment) | AE Layer FX · Perspective | EffectCraft Perspective · CC Environment (Environment, Mapping, Filter Environment, Height) |
| `ec.perspective.ccsphere` | CC Sphere (ec.perspective.ccsphere) | AE Layer FX · Perspective | EffectCraft Perspective · CC Sphere (Radius, Offset, Rotation X, Rotation Y) |
| `ec.perspective.ccspotlight` | CC Spotlight (ec.perspective.ccspotlight) | AE Layer FX · Perspective | EffectCraft Perspective · CC Spotlight (From, To, Height, Cone Angle) |
| `ec.perspective.dropshadow` | Drop Shadow (ec.perspective.dropshadow) | AE Layer FX · Perspective | EffectCraft Perspective · Drop Shadow (Shadow Color, Opacity, Direction, Distance) |
| `ec.perspective.radialshadow` | Radial Shadow (ec.perspective.radialshadow) | AE Layer FX · Perspective | EffectCraft Perspective · Radial Shadow (Shadow Color, Opacity, Light Source, Projection Distance) |
| `ec.sim.ccballaction` | CC Ball Action (ec.sim.ccballaction) | AE Layer FX · Simulation | EffectCraft Simulation · CC Ball Action (Scatter, Rotation Axis, Rotation, Twist Property) |
| `ec.sim.ccbubbles` | CC Bubbles (ec.sim.ccbubbles) | AE Layer FX · Simulation | EffectCraft Simulation · CC Bubbles (Bubble Amount, Bubble Speed, Wobble Amplitude, Wobble Frequency) |
| `ec.sim.ccdrizzle` | CC Drizzle (ec.sim.ccdrizzle) | AE Layer FX · Simulation | EffectCraft Simulation · CC Drizzle (Drip Rate, Longevity (sec), Rippling, Displacement) |
| `ec.sim.cchair` | CC Hair (ec.sim.cchair) | AE Layer FX · Simulation | EffectCraft Simulation · CC Hair (Length, Thickness, Weight, Constant Mass) |
| `ec.sim.ccmrmercury` | CC Mr. Mercury (ec.sim.ccmrmercury) | AE Layer FX · Simulation | EffectCraft Simulation · CC Mr. Mercury (Radius X, Radius Y, Producer, Direction) |
| `ec.sim.ccparticlesystems2` | CC Particle Systems II (ec.sim.ccparticlesystems2) | AE Layer FX · Simulation | EffectCraft Simulation · CC Particle Systems II (Birth Rate, Longevity (sec), Position, Radius X) |
| `ec.sim.ccparticleworld` | CC Particle World (ec.sim.ccparticleworld) | AE Layer FX · Simulation | EffectCraft Simulation · CC Particle World (Birth Rate, Longevity (sec), Position X, Position Y) |
| `ec.sim.ccpixelpolly` | CC Pixel Polly (ec.sim.ccpixelpolly) | AE Layer FX · Simulation | EffectCraft Simulation · CC Pixel Polly (Force, Gravity, Spinning, Force Center) |
| `ec.sim.ccrainfall` | CC Rainfall (ec.sim.ccrainfall) | AE Layer FX · Simulation | EffectCraft Simulation · CC Rainfall (Drops, Size, Scene Depth, Speed) |
| `ec.sim.ccscatterize` | CC Scatterize (ec.sim.ccscatterize) | AE Layer FX · Simulation | EffectCraft Simulation · CC Scatterize (Scatter, Right Twist, Left Twist, Transfer Mode) |
| `ec.sim.ccsnowfall` | CC Snowfall (ec.sim.ccsnowfall) | AE Layer FX · Simulation | EffectCraft Simulation · CC Snowfall (Flakes, Size, Variation % (Size), Scene Depth) |
| `ec.sim.ccstarburst` | CC Star Burst (ec.sim.ccstarburst) | AE Layer FX · Simulation | EffectCraft Simulation · CC Star Burst (Scatter, Speed, Phase, Grid Spacing) |
| `ec.sim.carddance` | Card Dance (ec.sim.carddance) | AE Layer FX · Simulation | EffectCraft Simulation · Card Dance (Rows & Columns, Rows, Columns, Back Layer) |
| `ec.sim.caustics` | Caustics (ec.sim.caustics) | AE Layer FX · Simulation | EffectCraft Simulation · Caustics (Bottom, Scaling, Repeat Mode, If Layer Size Differs) |
| `ec.sim.foam` | Foam (ec.sim.foam) | AE Layer FX · Simulation | EffectCraft Simulation · Foam (View, Producer Point, Producer X Size, Producer Y Size) |
| `ec.sim.particleplayground` | Particle Playground (ec.sim.particleplayground) | AE Layer FX · Simulation | EffectCraft Simulation · Particle Playground (Cannon Text, Grid Text, Auto-Orient Rotation, Cannon Enabled) |
| `ec.sim.shatter` | Shatter (ec.sim.shatter) | AE Layer FX · Simulation | EffectCraft Simulation · Shatter (View, Render, Pattern, Repetitions) |
| `ec.sim.waveworld` | Wave World (ec.sim.waveworld) | AE Layer FX · Simulation | EffectCraft Simulation · Wave World (View, Horizontal Rotation, Vertical Rotation, Vertical Scale) |
| `ec.stylize.brushstrokes` | Brush Strokes (ec.stylize.brushstrokes) | AE Layer FX · Stylize | EffectCraft Stylize · Brush Strokes (Stroke Angle, Brush Size, Stroke Length, Stroke Density) |
| `ec.stylize.ccblockload` | CC Block Load (ec.stylize.ccblockload) | AE Layer FX · Stylize | EffectCraft Stylize · CC Block Load (Completion, Scanlines, Smoothing, Start Cleared) |
| `ec.stylize.ccburnfilm` | CC Burn Film (ec.stylize.ccburnfilm) | AE Layer FX · Stylize | EffectCraft Stylize · CC Burn Film (Burn, Center, Random Seed) |
| `ec.stylize.ccglass` | CC Glass (ec.stylize.ccglass) | AE Layer FX · Stylize | EffectCraft Stylize · CC Glass (Bump Map, Property, Softness, Height) |
| `ec.stylize.cchextile` | CC HexTile (ec.stylize.cchextile) | AE Layer FX · Stylize | EffectCraft Stylize · CC HexTile (Render, Radius, Center, Rotate) |
| `ec.stylize.cckaleida` | CC Kaleida (ec.stylize.cckaleida) | AE Layer FX · Stylize | EffectCraft Stylize · CC Kaleida (Center, Size, Mirroring, Rotation) |
| `ec.stylize.ccmrsmoothie` | CC Mr. Smoothie (ec.stylize.ccmrsmoothie) | AE Layer FX · Stylize | EffectCraft Stylize · CC Mr. Smoothie (Flow Layer, Property, Smoothness, Sample A) |
| `ec.stylize.ccplastic` | CC Plastic (ec.stylize.ccplastic) | AE Layer FX · Stylize | EffectCraft Stylize · CC Plastic (Bump Layer, Property, Softness, Height) |
| `ec.stylize.ccrepetile` | CC RepeTile (ec.stylize.ccrepetile) | AE Layer FX · Stylize | EffectCraft Stylize · CC RepeTile (Expand Right, Expand Left, Expand Down, Expand Up) |
| `ec.stylize.ccthreshold` | CC Threshold (ec.stylize.ccthreshold) | AE Layer FX · Stylize | EffectCraft Stylize · CC Threshold (Threshold, Channel, Invert, Blend w. Original) |
| `ec.stylize.ccthresholdrgb` | CC Threshold RGB (ec.stylize.ccthresholdrgb) | AE Layer FX · Stylize | EffectCraft Stylize · CC Threshold RGB (Red Threshold, Green Threshold, Blue Threshold, Invert Red Channel) |
| `ec.stylize.ccvignette` | CC Vignette (ec.stylize.ccvignette) | AE Layer FX · Stylize | EffectCraft Stylize · CC Vignette (Amount, Angle of View, Center, Pin Highlights) |
| `ec.stylize.cartoon` | Cartoon (ec.stylize.cartoon) | AE Layer FX · Stylize | EffectCraft Stylize · Cartoon (Render, Detail Radius, Detail Threshold, Shading Steps) |
| `ec.stylize.coloremboss` | Color Emboss (ec.stylize.coloremboss) | AE Layer FX · Stylize | EffectCraft Stylize · Color Emboss (Direction, Relief, Contrast, Blend With Original) |
| `ec.stylize.emboss` | Emboss (ec.stylize.emboss) | AE Layer FX · Stylize | EffectCraft Stylize · Emboss (Direction, Relief, Contrast, Blend With Original) |
| `ec.stylize.findedges` | Find Edges (ec.stylize.findedges) | AE Layer FX · Stylize | EffectCraft Stylize · Find Edges (Invert, Blend With Original) |
| `ec.stylize.glow` | Glow (ec.stylize.glow) | AE Layer FX · Stylize | EffectCraft Stylize · Glow (Glow Based On, Glow Threshold, Glow Radius, Glow Intensity) |
| `ec.stylize.mosaic` | Mosaic (ec.stylize.mosaic) | AE Layer FX · Stylize | EffectCraft Stylize · Mosaic (Horizontal Blocks, Vertical Blocks, Sharp Colors) |
| `ec.stylize.motiontile` | Motion Tile (ec.stylize.motiontile) | AE Layer FX · Stylize | EffectCraft Stylize · Motion Tile (Tile Center, Tile Width, Tile Height, Output Width) |
| `ec.stylize.posterize` | Posterize (ec.stylize.posterize) | AE Layer FX · Stylize | EffectCraft Stylize · Posterize (Level) |
| `ec.stylize.roughenedges` | Roughen Edges (ec.stylize.roughenedges) | AE Layer FX · Stylize | EffectCraft Stylize · Roughen Edges (Edge Type, Edge Color, Border, Edge Sharpness) |
| `ec.stylize.scatter` | Scatter (ec.stylize.scatter) | AE Layer FX · Stylize | EffectCraft Stylize · Scatter (Scatter Amount, Grain, Scatter Randomness) |
| `ec.stylize.strobe` | Strobe Light (ec.stylize.strobe) | AE Layer FX · Stylize | EffectCraft Stylize · Strobe Light (Strobe Color, Blend With Original, Strobe Duration (secs), Strobe Period (secs)) |
| `ec.stylize.texturize` | Texturize (ec.stylize.texturize) | AE Layer FX · Stylize | EffectCraft Stylize · Texturize (Texture Layer, Light Direction, Texture Contrast, Texture Placement) |
| `ec.stylize.threshold` | Threshold (ec.stylize.threshold) | AE Layer FX · Stylize | EffectCraft Stylize · Threshold (Level) |
| `ec.text.numbers` | Numbers (ec.text.numbers) | AE Layer FX · Text | EffectCraft Text · Numbers (Type, Random Values, Value/Offset/Random Max, Decimal Places) |
| `ec.text.timecode` | Timecode (ec.text.timecode) | AE Layer FX · Text | EffectCraft Text · Timecode (Display Format, Time Source, Time Units, Drop Frame) |
| `ec.time.ccforcemotionblur` | CC Force Motion Blur (ec.time.ccforcemotionblur) | AE Layer FX · Time | EffectCraft Time · CC Force Motion Blur (Motion Blur Levels, Override Shutter Angle, Shutter Angle, Native Motion Blur) |
| `ec.time.ccwidetime` | CC Wide Time (ec.time.ccwidetime) | AE Layer FX · Time | EffectCraft Time · CC Wide Time (Forward Steps, Backward Steps, Native Motion Blur) |
| `ec.time.echo` | Echo (ec.time.echo) | AE Layer FX · Time | EffectCraft Time · Echo (Echo Time (seconds), Number Of Echoes, Starting Intensity, Decay) |
| `ec.time.pixelmotionblur` | Pixel Motion Blur (ec.time.pixelmotionblur) | AE Layer FX · Time | EffectCraft Time · Pixel Motion Blur (Shutter Control, Shutter Angle, Shutter Samples, Vector Detail) |
| `ec.time.posterizetime` | Posterize Time (ec.time.posterizetime) | AE Layer FX · Time | EffectCraft Time · Posterize Time (Frame Rate) |
| `ec.time.timedifference` | Time Difference (ec.time.timedifference) | AE Layer FX · Time | EffectCraft Time · Time Difference (Target, Time Offset (sec), Contrast, Absolute Difference) |
| `ec.time.timedisplacement` | Time Displacement (ec.time.timedisplacement) | AE Layer FX · Time | EffectCraft Time · Time Displacement (Time Displacement Layer, Max Displacement Time (sec), Time Resolution (fps), Stretch Map To Fit) |
| `ec.time.timewarp` | Timewarp (ec.time.timewarp) | AE Layer FX · Time | EffectCraft Time · Timewarp (Method, Adjust Time By, Speed, Source Frame) |
| `ec.transition.blockdissolve` | Block Dissolve (ec.transition.blockdissolve) | AE Layer FX · Transition | EffectCraft Transition · Block Dissolve (Transition Completion, Block Width, Block Height, Feather) |
| `ec.transition.ccglasswipe` | CC Glass Wipe (ec.transition.ccglasswipe) | AE Layer FX · Transition | EffectCraft Transition · CC Glass Wipe (Completion, Layer to Reveal, Gradient Layer, Softness) |
| `ec.transition.ccgridwipe` | CC Grid Wipe (ec.transition.ccgridwipe) | AE Layer FX · Transition | EffectCraft Transition · CC Grid Wipe (Completion, Center, Rotation, Border) |
| `ec.transition.ccimagewipe` | CC Image Wipe (ec.transition.ccimagewipe) | AE Layer FX · Transition | EffectCraft Transition · CC Image Wipe (Completion, Border Softness, Auto Softness, Layer) |
| `ec.transition.ccjaws` | CC Jaws (ec.transition.ccjaws) | AE Layer FX · Transition | EffectCraft Transition · CC Jaws (Completion, Center, Direction, Height) |
| `ec.transition.cclightwipe` | CC Light Wipe (ec.transition.cclightwipe) | AE Layer FX · Transition | EffectCraft Transition · CC Light Wipe (Completion, Center, Intensity, Shape) |
| `ec.transition.cclinesweep` | CC Line Sweep (ec.transition.cclinesweep) | AE Layer FX · Transition | EffectCraft Transition · CC Line Sweep (Completion, Direction, Thickness, Slant) |
| `ec.transition.ccradialscalewipe` | CC Radial ScaleWipe (ec.transition.ccradialscalewipe) | AE Layer FX · Transition | EffectCraft Transition · CC Radial ScaleWipe (Completion, Center, Reverse Transition) |
| `ec.transition.ccscalewipe` | CC Scale Wipe (ec.transition.ccscalewipe) | AE Layer FX · Transition | EffectCraft Transition · CC Scale Wipe (Stretch, Center, Direction) |
| `ec.transition.cctwister` | CC Twister (ec.transition.cctwister) | AE Layer FX · Transition | EffectCraft Transition · CC Twister (Completion, Backside, Shading, Center) |
| `ec.transition.ccwarpomatic` | CC WarpoMatic (ec.transition.ccwarpomatic) | AE Layer FX · Transition | EffectCraft Transition · CC WarpoMatic (Completion, Layer to Reveal, Reactor Layer, Reactor) |
| `ec.transition.cardwipe` | Card Wipe (ec.transition.cardwipe) | AE Layer FX · Transition | EffectCraft Transition · Card Wipe (Transition Completion, Transition Width, Back Layer, Back Layer Is Self) |
| `ec.transition.gradientwipe` | Gradient Wipe (ec.transition.gradientwipe) | AE Layer FX · Transition | EffectCraft Transition · Gradient Wipe (Transition Completion, Transition Softness, Gradient Layer, Gradient Placement) |
| `ec.transition.iriswipe` | Iris Wipe (ec.transition.iriswipe) | AE Layer FX · Transition | EffectCraft Transition · Iris Wipe (Iris Center, Iris Points, Outer Radius, Use Inner Radius) |
| `ec.transition.linearwipe` | Linear Wipe (ec.transition.linearwipe) | AE Layer FX · Transition | EffectCraft Transition · Linear Wipe (Transition Completion, Wipe Angle, Feather) |
| `ec.transition.radialwipe` | Radial Wipe (ec.transition.radialwipe) | AE Layer FX · Transition | EffectCraft Transition · Radial Wipe (Transition Completion, Start Angle, Wipe Center, Wipe) |
| `ec.transition.venetian` | Venetian Blinds (ec.transition.venetian) | AE Layer FX · Transition | EffectCraft Transition · Venetian Blinds (Transition Completion, Direction, Width, Feather) |
| `ec.utility.applylut` | Apply Color LUT (ec.utility.applylut) | AE Layer FX · Utility | EffectCraft Utility · Apply Color LUT (LUT) |
| `ec.utility.ccoverbrights` | CC Overbrights (ec.utility.ccoverbrights) | AE Layer FX · Utility | EffectCraft Utility · CC Overbrights (Channel, Highlight Color) |
| `ec.utility.cineon` | Cineon Converter (ec.utility.cineon) | AE Layer FX · Utility | EffectCraft Utility · Cineon Converter (Conversion Type, 10 Bit Black Point, Internal Black Point, 10 Bit White Point) |
| `ec.utility.colorprofileconverter` | Color Profile Converter (ec.utility.colorprofileconverter) | AE Layer FX · Utility | EffectCraft Utility · Color Profile Converter (Input Profile, Linearize Input Profile, Output Profile, Linearize Output Profile) |
| `ec.utility.facemeasurements` | Face Measurements (ec.utility.facemeasurements) | AE Layer FX · Utility | EffectCraft Utility · Face Measurements (Head Position X, Head Position Y, Head Scale, Head Orientation X) |
| `ec.utility.facetrackpoints` | Face Track Points (ec.utility.facetrackpoints) | AE Layer FX · Utility | EffectCraft Utility · Face Track Points (Left Eyebrow Inner, Left Eyebrow Middle, Left Eyebrow Outer, Right Eyebrow Inner) |
| `ec.utility.growbounds` | Grow Bounds (ec.utility.growbounds) | AE Layer FX · Utility | EffectCraft Utility · Grow Bounds (Pixels) |
| `ec.utility.hdrcompander` | HDR Compander (ec.utility.hdrcompander) | AE Layer FX · Utility | EffectCraft Utility · HDR Compander (Mode, Gain, Gamma) |
| `ec.utility.hdrcompression` | HDR Highlight Compression (ec.utility.hdrcompression) | AE Layer FX · Utility | EffectCraft Utility · HDR Highlight Compression (Amount) |

---

## Phần B — 61 Hiệu ứng & Chuyển cảnh Âm thanh trong FilmCraft (`audio_effects`)

| Mã `id` | Tên hiệu ứng | Nhóm | Mô tả & Tham số chính |
|---|---|---|---|
| `volume` | Volume (volume) | Core Audio | Hiệu ứng âm thanh Volume · tham số: bypass, level |
| `channel_volume` | Channel Volume (channel_volume) | Core Audio | Hiệu ứng âm thanh Channel Volume · tham số: bypass, left, right |
| `panner` | Panner (panner) | Core Audio | Hiệu ứng âm thanh Panner · tham số: balance |
| `constant_power` | Constant Power (constant_power) | Crossfade | Hiệu ứng âm thanh Constant Power |
| `constant_gain` | Constant Gain (constant_gain) | Crossfade | Hiệu ứng âm thanh Constant Gain |
| `exponential_fade` | Exponential Fade (exponential_fade) | Crossfade | Hiệu ứng âm thanh Exponential Fade |
| `amplify` | Amplify (amplify) | Amplitude and Compression | Hiệu ứng âm thanh Amplify · tham số: gain |
| `channel_mixer_a` | Channel Mixer (channel_mixer_a) | Amplitude and Compression | Hiệu ứng âm thanh Channel Mixer · tham số: l_from_l, l_from_r, invert_l, r_from_l |
| `channel_volume_a` | Channel Volume (channel_volume_a) | Amplitude and Compression | Hiệu ứng âm thanh Channel Volume · tham số: left, right |
| `deesser` | DeEsser (deesser) | Amplitude and Compression | Hiệu ứng âm thanh DeEsser · tham số: frequency, threshold, reduction |
| `dynamics_rack` | Dynamics (dynamics_rack) | Amplitude and Compression | Hiệu ứng âm thanh Dynamics · tham số: gate_on, gate_threshold, gate_attack, gate_release |
| `dynamics` | Dynamics Processing (dynamics) | Amplitude and Compression | Hiệu ứng âm thanh Dynamics Processing · tham số: threshold, ratio, attack, release |
| `hard_limiter` | Hard Limiter (hard_limiter) | Amplitude and Compression | Hiệu ứng âm thanh Hard Limiter · tham số: max, boost, lookahead, release |
| `multiband_compressor` | Multiband Compressor (multiband_compressor) | Amplitude and Compression | Hiệu ứng âm thanh Multiband Compressor · tham số: xo1, xo2, xo3, b1_threshold |
| `single_band_compressor` | Single-band Compressor (single_band_compressor) | Amplitude and Compression | Hiệu ứng âm thanh Single-band Compressor · tham số: threshold, ratio, attack, release |
| `tube_compressor` | Tube-modeled Compressor (tube_compressor) | Amplitude and Compression | Hiệu ứng âm thanh Tube-modeled Compressor · tham số: threshold, ratio, attack, release |
| `analog_delay` | Analog Delay (analog_delay) | Delay and Echo | Hiệu ứng âm thanh Analog Delay · tham số: mode, dry, wet, delay |
| `delay` | Delay (delay) | Delay and Echo | Hiệu ứng âm thanh Delay · tham số: delay, feedback, mix |
| `multitap_delay` | Multitap Delay (multitap_delay) | Delay and Echo | Hiệu ứng âm thanh Multitap Delay · tham số: delay1, feedback1, level1, delay2 |
| `bandpass` | Bandpass (bandpass) | Filter and EQ | Hiệu ứng âm thanh Bandpass · tham số: center, q |
| `bass` | Bass (bass) | Filter and EQ | Hiệu ứng âm thanh Bass · tham số: boost |
| `fft_filter` | FFT Filter (fft_filter) | Filter and EQ | Hiệu ứng âm thanh FFT Filter · tham số: p1_freq, p1_gain, p2_freq, p2_gain |
| `graphic_eq` | Graphic Equalizer (10 Bands) (graphic_eq) | Filter and EQ | Hiệu ứng âm thanh Graphic Equalizer (10 Bands) · tham số: b1, b2, b3, b4 |
| `graphic_eq_20` | Graphic Equalizer (20 Bands) (graphic_eq_20) | Filter and EQ | Hiệu ứng âm thanh Graphic Equalizer (20 Bands) · tham số: b1, b2, b3, b4 |
| `graphic_eq_30` | Graphic Equalizer (30 Bands) (graphic_eq_30) | Filter and EQ | Hiệu ứng âm thanh Graphic Equalizer (30 Bands) · tham số: b1, b2, b3, b4 |
| `highpass` | Highpass (highpass) | Filter and EQ | Hiệu ứng âm thanh Highpass · tham số: cutoff |
| `lowpass` | Lowpass (lowpass) | Filter and EQ | Hiệu ứng âm thanh Lowpass · tham số: cutoff |
| `notch` | Notch Filter (notch) | Filter and EQ | Hiệu ứng âm thanh Notch Filter · tham số: n1_on, n1_freq, n1_gain, n2_on |
| `parametric_eq` | Parametric Equalizer (parametric_eq) | Filter and EQ | Hiệu ứng âm thanh Parametric Equalizer · tham số: master_gain, hp_on, hp_freq, hp_slope |
| `scientific_filter` | Scientific Filter (scientific_filter) | Filter and EQ | Hiệu ứng âm thanh Scientific Filter · tham số: type, mode, order, cutoff |
| `simple_notch` | Simple Notch Filter (simple_notch) | Filter and EQ | Hiệu ứng âm thanh Simple Notch Filter · tham số: center, q |
| `simple_eq` | Simple Parametric EQ (simple_eq) | Filter and EQ | Hiệu ứng âm thanh Simple Parametric EQ · tham số: center, q, boost |
| `treble` | Treble (treble) | Filter and EQ | Hiệu ứng âm thanh Treble · tham số: boost |
| `chorus_flanger` | Chorus/Flanger (chorus_flanger) | Modulation | Hiệu ứng âm thanh Chorus/Flanger · tham số: mode, speed, width, intensity |
| `flanger` | Flanger (flanger) | Modulation | Hiệu ứng âm thanh Flanger · tham số: initial_delay, final_delay, stereo_phasing, feedback |
| `phaser` | Phaser (phaser) | Modulation | Hiệu ứng âm thanh Phaser · tham số: stages, intensity, depth, rate |
| `declicker` | Automatic Click Remover (declicker) | Noise Reduction/Restoration | Hiệu ứng âm thanh Automatic Click Remover · tham số: threshold, complexity |
| `dehummer` | DeHummer (dehummer) | Noise Reduction/Restoration | Hiệu ứng âm thanh DeHummer · tham số: freq, gain, harmonics, q |
| `denoise` | DeNoise (denoise) | Noise Reduction/Restoration | Hiệu ứng âm thanh DeNoise · tham số: amount |
| `dereverb` | DeReverb (dereverb) | Noise Reduction/Restoration | Hiệu ứng âm thanh DeReverb · tham số: amount, rt60 |
| `speech_enhance` | Enhance Speech (speech_enhance) | Noise Reduction/Restoration | Hiệu ứng âm thanh Enhance Speech · tham số: mix, tone |
| `convolution_reverb` | Convolution Reverb (convolution_reverb) | Reverb | Hiệu ứng âm thanh Convolution Reverb · tham số: impulse, mix, room_size, damping_lf |
| `studio_reverb` | Studio Reverb (studio_reverb) | Reverb | Hiệu ứng âm thanh Studio Reverb · tham số: room, decay, damping, dry |
| `surround_reverb` | Surround Reverb (surround_reverb) | Reverb | Hiệu ứng âm thanh Surround Reverb · tham số: center_input, room_size, decay, predelay |
| `binauralizer` | Binauralizer - Ambisonics (binauralizer) | Special | Hiệu ứng âm thanh Binauralizer - Ambisonics · tham số: angle, head_size, mix |
| `distortion` | Distortion (distortion) | Special | Hiệu ứng âm thanh Distortion · tham số: drive, curve, symmetry, tone |
| `fill_left` | Fill Left with Right (fill_left) | Special | Hiệu ứng âm thanh Fill Left with Right |
| `fill_right` | Fill Right with Left (fill_right) | Special | Hiệu ứng âm thanh Fill Right with Left |
| `guitar_suite` | GuitarSuite (guitar_suite) | Special | Hiệu ứng âm thanh GuitarSuite · tham số: compressor, distortion, dist_type, amp |
| `invert_a` | Invert (invert_a) | Special | Hiệu ứng âm thanh Invert |
| `loudness_radar` | Loudness Meter (loudness_radar) | Special | Hiệu ứng âm thanh Loudness Meter · tham số: target |
| `mastering` | Mastering (mastering) | Special | Hiệu ứng âm thanh Mastering · tham số: eq_low, eq_mid_freq, eq_mid, eq_high |
| `panner_ambisonics` | Panner - Ambisonics (panner_ambisonics) | Special | Hiệu ứng âm thanh Panner - Ambisonics · tham số: pan, tilt, roll |
| `swap_channels` | Swap Channels (swap_channels) | Special | Hiệu ứng âm thanh Swap Channels |
| `vocal_enhancer` | Vocal Enhancer (vocal_enhancer) | Special | Hiệu ứng âm thanh Vocal Enhancer · tham số: mode |
| `stereo_expander` | Stereo Expander (stereo_expander) | Stereo Imagery | Hiệu ứng âm thanh Stereo Expander · tham số: center_pan, expand |
| `stereo_width` | Stereo Width (stereo_width) | Stereo Imagery | Hiệu ứng âm thanh Stereo Width · tham số: width |
| `pitch_shifter` | Pitch Shifter (pitch_shifter) | Time and Pitch | Hiệu ứng âm thanh Pitch Shifter · tham số: semitones, cents |
| `balance_a` | Balance (balance_a) | Core Audio | Hiệu ứng âm thanh Balance · tham số: balance |
| `mute` | Mute (mute) | Core Audio | Hiệu ứng âm thanh Mute · tham số: mute |
| `volume_a` | Volume (volume_a) | Core Audio | Hiệu ứng âm thanh Volume · tham số: bypass, level |
