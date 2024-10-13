# Auto Wallpaper

Auto Wallpaper is a simple tool designed to automatically change your desktop wallpaper at a specified interval. The current implementation allows you to configure how often to change the wallpaper and where to pull them from. 

A config.toml file will be generated in APPDATA, under the folder wallpaperChanger. No support for linux at this time, although it should work with some simple tweaking.

## TODO List
- [ ] **Add support for `time_of_day` mode**: Change the wallpaper based on the time of day (morning, afternoon, evening, night).
- [ ] **Add category-based wallpaper selection**: Implement logic for changing wallpapers based on user-defined categories (nature, space, etc.).
- [ ] **Implement `monthly` mode**: Switch wallpapers based on the current month or season (winter, spring, summer, fall).
- [ ] **Support for multiple screen layouts**: Enhance functionality for multi-monitor setups (e.g., different wallpapers on each monitor).
- [ ] **Add interval-based wallpaper change**: Ensure the wallpaper is updated automatically after a specified interval.
- [ ] **Improve caching mechanism**: Optimize the image caching process to further reduce filesystem read times.
- [ ] **Improve logging**: Add more detailed logging, especially for error handling and wallpaper changes.
- [ ] **Unit tests**: Write unit tests for core functionality.
