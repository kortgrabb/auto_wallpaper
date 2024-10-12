# Auto Wallpaper

Auto Wallpaper is a simple tool I created to keep my desktop fresh, as I tend to get tired of wallpapers quickly. It searches your `/Pictures` folder for a subfolder named `wallpapers`, and within it, finds seasonal folders (`winter`, `spring`, `summer`, `autumn`) to pick a random image based on the current season.

To automate wallpaper changes, you need to create a task that runs the binary based on an event of your choice. Personally, I have it set to run every time I unlock my computer.

## Creating a Task on Linux

To set up an automatic task on Linux, you can use `cron`:

1. Open a terminal and type `crontab -e` to edit your crontab file.
2. Add a line to schedule the wallpaper change. For example, to run the script every time the computer starts, add:
   ```
   @reboot /path/to/your/auto_wallpaper_binary
   ```
   Or to run it every hour:
   ```
   0 * * * * /path/to/your/auto_wallpaper_binary
   ```
3. Save and exit the editor.

Alternatively, you can use a systemd service to trigger the task based on specific events, like waking up from sleep.

## Creating a Task on Windows

To automate wallpaper changes on Windows, you can use Task Scheduler:

1. Open **Task Scheduler** by searching for it in the Start menu.
2. Click on **Create Basic Task**.
3. Name your task (e.g., "Auto Wallpaper Change") and click **Next**.
4. Choose the trigger for the task. For example, select **When I log on** or **When the computer unlocks**.
5. Click **Next**, then select **Start a program**.
6. Browse for your binary (`auto_wallpaper.exe`) and click **Next**.
7. Click **Finish** to create the task.

## Feature Checklist

### Features Implemented
- Randomly select a wallpaper based on the current season.
- Support for organizing wallpapers into seasonal folders (`winter`, `spring`, `summer`, `autumn`).
- Instructions for automating wallpaper changes on Linux and Windows.

### Features to Add
- [ ] Support for additional events (e.g., change wallpaper every hour).
- [ ] Configurable wallpaper folder paths.
- [ ] GUI to easily configure settings.
- [ ] Cross-platform notifications when the wallpaper changes.
- [ ] Option to shuffle between all wallpapers regardless of season.
- [ ] Logging to track wallpaper changes and any errors.
- [ ] Integration with online sources for automatic wallpaper downloads.