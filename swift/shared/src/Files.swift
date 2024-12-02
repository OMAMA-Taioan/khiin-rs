import Foundation

func getAppDataDir() -> URL? {
    let fileManager = FileManager.default

    let appSupportDir = fileManager.urls(
        for: .applicationSupportDirectory, in: .userDomainMask
    ).first!

    let dataDir = appSupportDir.appendingPathComponent("app.khiin.KhiinPJH")

    if let _ = try? FileManager.default.createDirectory(
        at: dataDir, withIntermediateDirectories: true)
    {
        return dataDir.absoluteURL
    }

    return nil
}

func getDatabaseFilePath() -> String? {
    if let dbPath = Bundle.main.path(forResource: "khiin", ofType: "db") {
        return dbPath
    }

    return nil
}

func getSettingFilePath() -> String? {
    if let dataDir = getAppDataDir() {
        return dataDir.appendingPathComponent("settings.toml").absoluteURL
            .path(percentEncoded: false)
    }

    return nil
}
