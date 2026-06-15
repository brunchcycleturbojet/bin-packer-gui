import { RiSunLine, RiMoonLine, RiComputerLine } from "@remixicon/react"

import { Button } from "#components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "#components/ui/dropdown-menu"
import { useTheme } from "#components/theme-provider"

export function ModeToggle() {
  const { theme, setTheme } = useTheme()

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="secondary" size="icon">
          {theme == "dark" && 
            <RiMoonLine className="absolute h-[1.2rem] w-[1.2rem]" />
          }
          {theme == "light" && 
            <RiSunLine className="h-[1.2rem] w-[1.2rem]" />
          }
          {theme == "system" && 
            <RiComputerLine className="h-[1.2rem] w-[1.2rem]" />
          }
          
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => setTheme("light")}>
          <RiSunLine className="mr-2 h-4 w-4" />
          Light
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setTheme("dark")}>
          <RiMoonLine className="mr-2 h-4 w-4" />
          Dark
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setTheme("system")}>
          <RiComputerLine className="mr-2 h-4 w-4" />
          System
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
