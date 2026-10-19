from pathlib import Path
import unittest

from playwright.sync_api import expect, sync_playwright


ROOT = Path(__file__).resolve().parents[2]


class OfflineGameTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.playwright = sync_playwright().start()
        cls.browser = cls.playwright.chromium.launch(
            args=["--use-angle=swiftshader", "--enable-unsafe-swiftshader"]
        )

    @classmethod
    def tearDownClass(cls):
        cls.browser.close()
        cls.playwright.stop()

    def setUp(self):
        self.context = self.browser.new_context(
            viewport={"width": 1440, "height": 1100}, offline=True
        )
        self.context.tracing.start(screenshots=True, snapshots=True, sources=True)
        self.page = self.context.new_page()
        self.errors = []
        self.external_requests = []
        self.page.on("pageerror", lambda error: self.errors.append(str(error)))
        self.page.on("console", lambda message: self.errors.append(message.text)
                     if message.type == "error" else None)
        self.page.on("request", lambda request: self.external_requests.append(request.url)
                     if request.url.startswith(("http://", "https://")) else None)
        self.page.goto((ROOT / "dist/play.html").as_uri())
        self.status = self.page.locator("#game-status")
        expect(self.status).to_have_attribute("data-phase", "0", timeout=30_000)
        expect(self.page.locator("#loading")).to_be_hidden()
        expect(self.page.locator("#load-error")).to_be_hidden()
        self.page.locator("canvas").focus()

    def tearDown(self):
        diagnostics = ROOT / "test-results" / self.id().rsplit(".", 1)[-1]
        diagnostics.mkdir(parents=True, exist_ok=True)
        try:
            self.page.screenshot(path=str(diagnostics / "screen.png"), full_page=True)
            self.context.tracing.stop(path=str(diagnostics / "trace.zip"))
            self.assertEqual(self.errors, [])
            self.assertEqual(self.external_requests, [])
        finally:
            self.context.close()

    def state(self):
        return self.status.evaluate(
            "element => Object.fromEntries(Object.entries(element.dataset)"
            ".map(([key, value]) => [key, Number(value)]))"
        )

    def wait_for(self, condition):
        self.page.wait_for_function(
            "() => { const s = Object.fromEntries(Object.entries("
            "document.querySelector('#game-status').dataset)"
            ".map(([key, value]) => [key, Number(value)])); return " + condition + "; }",
            timeout=20_000,
        )

    def start(self):
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-phase", "1")

    def queue(self, keys):
        slots = self.state()["slots"]
        for key in keys:
            slots += 2 if key in "23" else 1
            self.page.keyboard.press(key)
            expect(self.status).to_have_attribute("data-slots", str(slots))

    def test_planning_capacity_undo_and_fixed_enemy_intent(self):
        self.start()
        intent = self.state()["enemyPlan"]
        self.queue("2")
        self.page.keyboard.press("2")
        self.page.wait_for_timeout(100)
        self.assertEqual(self.state()["slots"], 2)
        self.assertEqual(self.state()["enemyPlan"], intent)
        self.page.keyboard.press("Backspace")
        expect(self.status).to_have_attribute("data-slots", "0")
        self.queue("16")
        self.page.keyboard.press("c")
        expect(self.status).to_have_attribute("data-slots", "0")
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-phase", "1")

    def test_two_series_movement_attack_and_health(self):
        self.start()
        self.queue("444")
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-phase", "2")
        self.wait_for("s.phase === 1 && s.round === 2")
        state = self.state()
        self.assertEqual((state["playerCell"], state["enemyCell"]), (2, 4))
        self.assertEqual(state["playerEnergy"], 6)
        self.queue("38")
        self.page.keyboard.press("Enter")
        self.wait_for("s.phase === 1 && s.round === 3")
        self.assertEqual(self.state()["playerHealth"], 21)
        self.assertEqual(self.state()["enemyHealth"], 12)

    def test_pause_blur_resume_and_restart(self):
        self.start()
        self.queue("444")
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-phase", "2")
        self.page.keyboard.press("p")
        expect(self.status).to_have_attribute("data-paused", "1")
        paused = self.state()
        self.page.wait_for_timeout(300)
        self.assertEqual(self.state(), paused)
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-paused", "0")
        self.page.evaluate("window.dispatchEvent(new Event('blur'))")
        expect(self.status).to_have_attribute("data-paused", "1")
        self.page.keyboard.press("r")
        expect(self.status).to_have_attribute("data-phase", "1")
        state = self.state()
        self.assertEqual((state["round"], state["slots"], state["paused"]), (1, 0, 0))
        self.assertEqual((state["playerHealth"], state["enemyHealth"]), (24, 16))

    def test_mouse_cards_and_help(self):
        self.start()
        self.page.keyboard.press("h")
        expect(self.status).to_have_attribute("data-help", "1")
        self.page.keyboard.press("1")
        self.assertEqual(self.state()["slots"], 0)
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-help", "0")
        box = self.page.locator("canvas").bounding_box()
        self.page.locator("canvas").click(position={"x": 150 / 1280 * box["width"], "y": 745 / 920 * box["height"]})
        expect(self.status).to_have_attribute("data-queued", "1")


if __name__ == "__main__":
    unittest.main()
