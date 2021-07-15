import * as React from "react";

import { Status } from "./Status";
import { UI } from "./UI";

export const Info = () => {
  return (
    <div className="container p-4">
      <UI />
      <Status />
    </div>
  );
};
